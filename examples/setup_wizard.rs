// examples/setup_wizard.rs
// A realistic setup wizard that uses every VelvetIO feature
// Run: cargo run --example setup_wizard

use velvetio::prelude::*;

fn main() {
    println!("🚀 Development Environment Setup Wizard\n");

    // Basic ask! macro usage
    let dev_name = ask!("Developer name");
    let team_size = ask!("Team size" => usize);

    // ask! with validation
    let email = ask!(
        "Email address",
        validate: |e: &String| e.contains('@'),
        error: "Enter a valid email"
    );

    // ask! with default values
    let editor = ask!("Preferred editor", default: "vscode".to_string());
    let dev_port = ask!("Default dev port" => u16, default: 3000);

    // try_ask! for optional input with fallback
    let budget = ask!("Monthly budget (USD)" => f64, or: 0.0);

    // confirm! macro
    let use_docker = confirm!("Use Docker for development?");
    let setup_ci = confirm!("Setup CI/CD pipeline?");

    // choose! macro - single selection
    let os = choose!("Primary OS", ["Linux", "macOS", "Windows", "Other"]);

    let cloud_provider = choose!(
        "Cloud provider",
        ["AWS", "Google Cloud", "Azure", "Digital Ocean", "None"]
    );

    // multi_select! macro - multiple choices
    let languages = multi_select!(
        "Programming languages used",
        [
            "Rust",
            "Python",
            "JavaScript",
            "Go",
            "Java",
            "C++",
            "TypeScript"
        ]
    );

    let databases = multi_select!(
        "Databases needed",
        ["PostgreSQL", "MySQL", "Redis", "MongoDB", "SQLite"]
    );

    // Form builder - showcasing all field types
    println!("\n📝 Project Configuration");
    let project_config = form()
        .text("project_name", "Project name")
        .number("version_major", "Major version number")
        .boolean("open_source", "Open source project?")
        .choice("license", "License type", &["MIT", "Apache-2.0", "GPL-3.0"])
        .multi_choice(
            "platforms",
            "Target platforms",
            &["Web", "Mobile", "Desktop", "API", "CLI"],
        )
        .optional("description", "Project description")
        .validated_text(
            "repo_url",
            "Repository URL",
            |url| url.starts_with("https://"),
            "URL must start with https://",
        )
        .collect();

    // quick_form! macro for simple data collection
    let contact_info = quick_form! {
        "company" => "Company name",
        "website" => "Company website",
        "phone" => "Phone number"
    };

    // Demonstrate type parsing capabilities
    let coordinates: (f64, f64) = ask!("Office coordinates (lat,lng)" => (f64, f64));
    let tags: Vec<String> = ask!("Project tags (comma-separated)" => Vec<String>);
    let backup_email: Option<String> = ask!("Backup email (optional)" => Option<String>);
    let initial_char = ask!("Project initial" => char);

    // Advanced validation with built-in validators
    let username = ask!(
        "Admin username",
        validate: and(min_length(3), max_length(20)),
        error: "Username must be 3-20 characters"
    );

    let server_count = ask!(
        "Number of servers" => u32,
        validate: and(is_positive, in_range(1, 100)),
        error: "Must be between 1 and 100"
    );

    // Custom type with quick_parse! macro
    #[derive(Debug)]
    struct ProjectId(String);

    impl ProjectId {
        fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl std::str::FromStr for ProjectId {
        type Err = ();
        fn from_str(s: &str) -> std::result::Result<Self, ()> {
            if s.len() >= 3 && s.chars().all(|c| c.is_alphanumeric()) {
                Ok(ProjectId(s.to_string()))
            } else {
                Err(())
            }
        }
    }

    quick_parse!(ProjectId);

    let project_id = ask!("Project ID (alphanumeric, 3+ chars)" => ProjectId);

    // Complex validation with or logic
    let api_timeout = ask!(
        "API timeout (seconds)" => u32,
        validate: or(
            |&n| n == 30,  // Quick option
            |&n| n >= 60 && n <= 300  // Custom range
        ),
        error: "Use 30 for default, or 60-300 for custom"
    );

    // Conditional workflow based on previous answers
    let deployment_config = if use_docker {
        Some(
            form()
                .choice(
                    "container_registry",
                    "Container registry",
                    &["Docker Hub", "AWS ECR", "Google Container Registry"],
                )
                .text("image_name", "Base image name")
                .boolean("multi_stage", "Multi-stage build?")
                .collect(),
        )
    } else {
        None
    };

    // Show everything we collected
    println!("\n✅ Setup Complete!");
    println!("Developer: {} <{}>", dev_name, email);
    println!("Team size: {}", team_size);
    println!("Editor: {} (dev port: {})", editor, dev_port);
    println!("OS: {}", os);
    println!("Budget: ${:.2}", budget);
    println!("Languages: {}", languages.join(", "));

    if !databases.is_empty() {
        println!("Databases: {}", databases.join(", "));
    }

    println!(
        "Project: {} (ID: {})",
        project_config.get("project_name").unwrap(),
        project_id.as_str()
    );

    println!("Coordinates: {:?}", coordinates);
    println!("Tags: {:?}", tags);
    println!("Backup email: {:?}", backup_email);
    println!("Initial: {}", initial_char);

    println!("Admin username: {} (servers: {})", username, server_count);
    println!("API timeout: {}s", api_timeout);

    // Show contact info from quick_form
    for (key, value) in contact_info {
        println!("{}: {}", key, value);
    }

    if let Some(docker_config) = deployment_config {
        println!(
            "Docker registry: {}",
            docker_config.get("container_registry").unwrap()
        );
    }

    if setup_ci {
        println!("CI/CD will be configured for {}", cloud_provider);
    }

    println!("\n🎉 Ready to start coding!");
}
