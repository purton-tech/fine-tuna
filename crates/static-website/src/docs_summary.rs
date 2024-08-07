use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![
            Category {
                name: "Introducing Bionic".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Introduction",
                    description: "",
                    folder: "docs/",
                    markdown: include_str!("../content/docs/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
            Category {
                name: "Learn Bionic".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Managing Models",
                        description: "",
                        folder: "docs/guides/managing-models/",
                        markdown: include_str!("../content/docs/guides/managing-models/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Automating Document Upload",
                        description: "",
                        folder: "docs/guides/uploading-documents/",
                        markdown: include_str!(
                            "../content/docs/guides/uploading-documents/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Using the API",
                        description: "",
                        folder: "docs/guides/api/",
                        markdown: include_str!("../content/docs/guides/api/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Retrieval Augmented Generation".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "AI Assistants",
                        description: "",
                        folder: "docs/rag/rag/",
                        markdown: include_str!("../content/docs/rag/rag/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Managing Prompts",
                        description: "",
                        folder: "docs/rag/prompts/",
                        markdown: include_str!("../content/docs/rag/prompts/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Local Bionic".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Try it on a Laptop",
                        description: "",
                        folder: "docs/running-locally/docker-compose/",
                        markdown: include_str!(
                            "../content/docs/running-locally/docker-compose/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting to Ollama",
                        description: "",
                        folder: "docs/running-locally/ollama/",
                        markdown: include_str!("../content/docs/running-locally/ollama/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Deploying To Your Infrastructure".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Quick Install (Linux)",
                        description: "",
                        folder: "docs/on-premise/install-linux/",
                        markdown: include_str!("../content/docs/on-premise/install-linux/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Quick Install RKE2",
                        description: "",
                        folder: "docs/on-premise/install-rke2/",
                        markdown: include_str!("../content/docs/on-premise/install-rke2/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Install AWS",
                        description: "",
                        folder: "docs/on-premise/aws/",
                        markdown: include_str!("../content/docs/on-premise/aws/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Install Google Cloud",
                        description: "",
                        folder: "docs/on-premise/gcloud/",
                        markdown: include_str!("../content/docs/on-premise/gcloud/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting Data Sources",
                        description: "",
                        folder: "docs/on-premise/airbyte/",
                        markdown: include_str!("../content/docs/on-premise/airbyte/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Configure Email",
                        description: "",
                        folder: "docs/on-premise/email/",
                        markdown: include_str!("../content/docs/on-premise/email/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Upgrading Bionic",
                        description: "",
                        folder: "docs/on-premise/upgrades/",
                        markdown: include_str!("../content/docs/on-premise/upgrades/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Single Sign On",
                        description: "",
                        folder: "docs/on-premise/sso/",
                        markdown: include_str!("../content/docs/on-premise/sso/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Role Based Access Control",
                        description: "",
                        folder: "docs/on-premise/rbac/",
                        markdown: include_str!("../content/docs/on-premise/rbac/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting pgAdmin",
                        description: "",
                        folder: "docs/on-premise/pgadmin/",
                        markdown: include_str!("../content/docs/on-premise/pgadmin/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Cloudflare as Ingress",
                        description: "",
                        folder: "docs/on-premise/cloudflare/",
                        markdown: include_str!("../content/docs/on-premise/cloudflare/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Licencing Bionic",
                        description: "",
                        folder: "docs/on-premise/licencing/",
                        markdown: include_str!("../content/docs/on-premise/licencing/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting a Jupyter Notebook",
                        description: "",
                        folder: "docs/on-premise/jupyter/",
                        markdown: include_str!("../content/docs/on-premise/jupyter/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Visualising RAG",
                        description: "",
                        folder: "docs/on-premise/visual-rag/",
                        markdown: include_str!("../content/docs/on-premise/visual-rag/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
