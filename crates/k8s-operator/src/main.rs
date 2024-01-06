mod bionic;
mod crd;
mod finalizer;

use anyhow::{bail, Result};
use either::Either::{Left, Right};
use garde::Validate;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tracing::*;

use crd::{Bionic, BionicSpec, BionicStatus};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    core::crd::CustomResourceExt,
    Client,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let client = Client::try_default().await?;

    // Manage CRDs first
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());

    // Delete any old versions of it first:
    let dp = DeleteParams::default();
    // but ignore delete err if not exists
    let _ = crds.delete("bionics.bionic-gpt.com", &dp).await.map(|res| {
        res.map_left(|o| {
            info!(
                "Deleting {}: ({:?})",
                o.name_any(),
                o.status.unwrap().conditions.unwrap().last()
            );
        })
        .map_right(|s| {
            // it's gone.
            info!("Deleted bionics.bionic-gpt.com: ({:?})", s);
        })
    });
    // Wait for the delete to take place (map-left case or delete from previous run)
    sleep(Duration::from_secs(2)).await;

    // Create the CRD so we can create Bionics in kube
    let foocrd = Bionic::crd();
    info!(
        "Creating Bionic CRD: {}",
        serde_json::to_string_pretty(&foocrd)?
    );
    let pp = PostParams::default();
    let patch_params = PatchParams::default();
    match crds.create(&pp, &foocrd).await {
        Ok(o) => {
            info!("Created {} ({:?})", o.name_any(), o.status.unwrap());
            debug!("Created CRD: {:?}", o.spec);
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }
    // Wait for the api to catch up
    sleep(Duration::from_secs(1)).await;

    // Manage the Bionic CR
    let foos: Api<Bionic> = Api::default_namespaced(client.clone());

    // Create Bionic baz
    info!("Creating Bionic instance baz");
    let f1 = Bionic::new(
        "baz",
        BionicSpec {
            name: "baz".into(),
            info: "old baz".into(),
            replicas: 1,
        },
    );
    let o = foos.create(&pp, &f1).await?;
    assert_eq!(ResourceExt::name_any(&f1), ResourceExt::name_any(&o));
    info!("Created {}", o.name_any());

    // Verify we can get it
    info!("Get Bionic baz");
    let f1cpy = foos.get("baz").await?;
    assert_eq!(f1cpy.spec.info, "old baz");

    // Replace its spec
    info!("Replace Bionic baz");
    let foo_replace: Bionic = serde_json::from_value(json!({
        "apiVersion": "bionic-gpt.com/v1",
        "kind": "Bionic",
        "metadata": {
            "name": "baz",
            // Updates need to provide our last observed version:
            "resourceVersion": f1cpy.resource_version(),
        },
        "spec": { "name": "baz", "info": "new baz", "replicas": 1 },
    }))?;
    let f1_replaced = foos.replace("baz", &pp, &foo_replace).await?;
    assert_eq!(f1_replaced.spec.name, "baz");
    assert_eq!(f1_replaced.spec.info, "new baz");
    assert!(f1_replaced.status.is_none());

    // Delete it
    foos.delete("baz", &dp).await?.map_left(|f1del| {
        assert_eq!(f1del.spec.info, "old baz");
    });

    // Create Bionic qux with status
    info!("Create Bionic instance qux");
    let f2 = Bionic::new(
        "qux",
        BionicSpec {
            name: "qux".into(),
            replicas: 0,
            info: "unpatched qux".into(),
        },
    );

    let o = foos.create(&pp, &f2).await?;
    info!("Created {}", o.name_any());

    // Update status on qux (cannot be done through replace/create/patch direct)
    info!("Replace Status on Bionic instance qux");
    let fs = json!({
        "apiVersion": "bionic-gpt.com/v1",
        "kind": "Bionic",
        "metadata": {
            "name": "qux",
            // Updates need to provide our last observed version:
            "resourceVersion": o.resource_version(),
        },
        "status": BionicStatus { is_bad: true, replicas: 0 }
    });
    let o = foos
        .replace_status("qux", &pp, serde_json::to_vec(&fs)?)
        .await?;
    info!("Replaced status {:?} for {}", o.status, o.name_any());
    assert!(o.status.unwrap().is_bad);

    info!("Patch Status on Bionic instance qux");
    let fs = json!({
        "status": BionicStatus { is_bad: false, replicas: 1 }
    });
    let o = foos
        .patch_status("qux", &patch_params, &Patch::Merge(&fs))
        .await?;
    info!("Patched status {:?} for {}", o.status, o.name_any());
    assert!(!o.status.unwrap().is_bad);

    info!("Get Status on Bionic instance qux");
    let o = foos.get_status("qux").await?;
    info!("Got status {:?} for {}", o.status, o.name_any());
    assert!(!o.status.unwrap().is_bad);

    // Check scale subresource:
    info!("Get Scale on Bionic instance qux");
    let scale = foos.get_scale("qux").await?;
    info!("Got scale {:?} - {:?}", scale.spec, scale.status);
    assert_eq!(scale.status.unwrap().replicas, 1);

    // Scale up
    let fs = json!({
        "spec": { "replicas": 2 }
    });
    let o = foos
        .patch_scale("qux", &patch_params, &Patch::Merge(&fs))
        .await?;
    info!("Patched scale {:?} for {}", o.spec, o.name_any());
    assert_eq!(o.status.unwrap().replicas, 1);
    assert_eq!(o.spec.unwrap().replicas.unwrap(), 2); // we only asked for more

    // Modify a Bionic qux with a Patch
    info!("Patch Bionic instance qux");
    let patch = json!({
        "spec": { "info": "patched qux" }
    });
    let o = foos
        .patch("qux", &patch_params, &Patch::Merge(&patch))
        .await?;
    info!("Patched {} with new name: {}", o.name_any(), o.spec.name);
    assert_eq!(o.spec.info, "patched qux");
    assert_eq!(o.spec.name, "qux"); // didn't blat existing params

    // Check we have 1 remaining instance
    let lp = ListParams::default();
    let res = foos.list(&lp).await?;
    assert_eq!(res.items.len(), 1);

    // Delete the last - expect a status back (instant delete)
    assert!(foos.delete("qux", &dp).await?.is_right());

    // Check that validation is being obeyed
    info!("Verifying validation rules");
    let fx = Bionic::new(
        "x",
        BionicSpec {
            name: "x".into(),
            info: "failing validation obj".into(),
            replicas: 1,
        },
    );
    // using derived Validate rules locally:
    assert!(fx.spec.validate(&()).is_err());
    // check rejection from apiserver (validation rules embedded in JsonSchema)
    match foos.create(&pp, &fx).await {
        Err(kube::Error::Api(ae)) => {
            assert_eq!(ae.code, 422);
            assert!(ae
                .message
                .contains("spec.name in body should be at least 3 chars long"));
        }
        Err(e) => bail!("somehow got unexpected error from validation: {:?}", e),
        Ok(o) => bail!("somehow created {:?} despite validation", o),
    }
    info!("Rejected fx for invalid name {}", fx.name_any());

    // Cleanup the full collection - expect a wait
    match foos.delete_collection(&dp, &lp).await? {
        Left(list) => {
            let deleted: Vec<_> = list.iter().map(ResourceExt::name_any).collect();
            info!("Deleting collection of foos: {:?}", deleted);
        }
        Right(status) => {
            info!("Deleted collection of crds: status={:?}", status);
        }
    }

    // Cleanup the CRD definition
    match crds.delete("bionics.bionic-gpt.com", &dp).await? {
        Left(o) => {
            info!(
                "Deleting {} CRD definition: {:?}",
                o.name_any(),
                o.status.unwrap().conditions.unwrap().last()
            );
        }
        Right(status) => {
            info!("Deleted foos CRD definition: status={:?}", status);
        }
    }

    Ok(())
}
