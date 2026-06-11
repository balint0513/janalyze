pub mod cleanup_rules {
    pub struct CleanupRule {
        pub name: &'static str,
        pub anchor: &'static str,
        pub trash_targets: &'static [&'static str],
    }

    pub const RULES: &[CleanupRule] = &[
        CleanupRule {
            name: "Rust",
            anchor: "Cargo.toml",
            trash_targets: &["target"],
        },
        CleanupRule {
            name: "Node.js",
            anchor: "package.json",
            trash_targets: &["node_modules", "dist", ".next"],
        },
        CleanupRule {
            name: "Flutter",
            anchor: "pubspec.yaml",
            trash_targets: &["build", ".dart_tool"],
        },
        CleanupRule {
            name: "Python",
            anchor: "requirements.txt",
            trash_targets: &["__pycache__", "dist", "build"],
        },
        CleanupRule {
            name: "Java",
            anchor: "pom.xml",
            trash_targets: &["target"],
        },
        CleanupRule {
            name: "Go",
            anchor: "go.mod",
            trash_targets: &["bin", "pkg"],
        },
        CleanupRule {
            name: "C#",
            anchor: "*.csproj",
            trash_targets: &["bin", "obj"],
        },
        CleanupRule {
            name: "Django",
            anchor: "manage.py",
            trash_targets: &["__pycache__", "staticfiles", "media"],
        },
        CleanupRule {
            name: "React",
            anchor: "package.json",
            trash_targets: &["node_modules", "build"],
        },
         CleanupRule {
            name: "Vue.js",
            anchor: "package.json",
            trash_targets: &["node_modules", "dist"],
        },
         CleanupRule {
            name: "Angular",
            anchor: "angular.json",
            trash_targets: &["node_modules", "dist"],
        },
         CleanupRule {
            name: "Ruby on Rails",
            anchor: "Gemfile",
            trash_targets: &["log", "tmp"],
        },
         CleanupRule {
            name: "Laravel",
            anchor: "composer.json",
            trash_targets: &["vendor", "storage/framework/cache"],
        },
        CleanupRule {
            name: "Gradle",
            anchor: "build.gradle",
            trash_targets: &["build"],
        }
    ];
}

