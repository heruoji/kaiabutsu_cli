use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const CRAWLER_JAVA_TEMPLATE: &str = include_str!("../../templates/Crawler.java");
const BUILD_GRADLE_TEMPLATE: &str = include_str!("../../templates/build.gradle");
const SETTINGS_GRADLE_TEMPLATE: &str = include_str!("../../templates/settings.gradle");
const GRADLE_WRAPPER_PROPERTIES_TEMPLATE: &str = include_str!("../../templates/gradle-wrapper.properties");
const GITIGNORE_TEMPLATE: &str = include_str!("../../templates/.gitignore");
const APPLICATION_PROPERTIES_TEMPLATE: &str = include_str!("../../templates/application.properties");

// Quote Example
const QUOTE_PROPERTIES_TEMPLATE: &str = include_str!("../../templates/quote/quote.properties");
const QUOTE_ITEM_TEMPLATE: &str = include_str!("../../templates/quote/AuthorItem.java");
const QUOTE_SPIDER_TEMPLATE: &str = include_str!("../../templates/quote/QuoteSpider.java");
const QUOTE_ITEM_PIPELINE_TEMPLATE: &str = include_str!("../../templates/quote/CsvWriterAuthorItemPipeline.java");
const QUOTE_BUILD_GRADLE_TEMPLATE: &str = include_str!("../../templates/quote/build.gradle");

pub fn start_project(project_name: &str, package: &str, include_quote_example: bool) {
    let root_path = Path::new(project_name);
    create_dir(root_path.to_str().unwrap());
    let test_java_dir_path = root_path.join("src/test/java");
    create_dir(test_java_dir_path.to_str().unwrap());

    let file_creations = prepare_file_creations(&root_path, package, project_name, include_quote_example);

    for (file_path, content) in file_creations.iter() {
        if let Err(e) = create_file_from_template(file_path, content) {
            eprintln!("Failed to create file {}: {}", file_path.display(), e);
            return;
        }
    }
}
//
// fn prepare_file_creations<'a>(root_path: &'a Path, package: &'a str, project_name: &'a str, include_quote_example: bool) -> Vec<(PathBuf, &'a str)> {
//     if include_quote_example {
//         vec![
//             (root_path.join("src/main/java").join(package.replace(".", "/")).join("Crawler.java"), &CRAWLER_JAVA_TEMPLATE.replace("{{package}}", package)),
//             (root_path.join("src/main/resources").join(format!("{}.properties", project_name)), QUOTE_PROPERTIES_TEMPLATE),
//             (root_path.join(".gitignore"), GITIGNORE_TEMPLATE),
//             (root_path.join("build.gradle"), &QUOTE_BUILD_GRADLE_TEMPLATE.replace("{{group}}", package)),
//             (root_path.join("settings.gradle"), &SETTINGS_GRADLE_TEMPLATE.replace("{{project_name}}", project_name)),
//             (root_path.join("gradle/wrapper/gradle-wrapper.properties"), GRADLE_WRAPPER_PROPERTIES_TEMPLATE),
//             (root_path.join("src/main/java").join(package.replace(".", "/")).join("/spider/QuoteSpider.java"), &QUOTE_SPIDER_TEMPLATE.replace("{{package}}", package)),
//             (root_path.join("src/main/java").join(package.replace(".", "/")).join("/itempipeline/CsvWriterAuthorItemPipeline.java"), &QUOTE_ITEM_PIPELINE_TEMPLATE.replace("{{package}}", package)),
//             (root_path.join("src/main/java").join(package.replace(".", "/")).join("/item/AuthorItem.java"), &QUOTE_ITEM_TEMPLATE.replace("{{package}}", package)),
//         ]
//     } else {
//         vec![
//             (root_path.join("src/main/java").join(package.replace(".", "/")).join("Crawler.java"), &CRAWLER_JAVA_TEMPLATE.replace("{{package}}", package)),
//             (root_path.join("src/main/resources").join(format!("{}.properties", project_name)), APPLICATION_PROPERTIES_TEMPLATE),
//             (root_path.join(".gitignore"), GITIGNORE_TEMPLATE),
//             (root_path.join("build.gradle"), &BUILD_GRADLE_TEMPLATE.replace("{{group}}", package)),
//             (root_path.join("settings.gradle"), &SETTINGS_GRADLE_TEMPLATE.replace("{{project_name}}", project_name)),
//             (root_path.join("gradle/wrapper/gradle-wrapper.properties"), GRADLE_WRAPPER_PROPERTIES_TEMPLATE),
//         ]
//     }
// }

fn prepare_file_creations(root_path: &Path, package: &str, project_name: &str, include_quote_example: bool) -> Vec<(PathBuf, String)> {
    if include_quote_example {
        vec![
            (root_path.join("src/main/java").join(package.replace(".", "/")).join("Crawler.java"), CRAWLER_JAVA_TEMPLATE.replace("{{package}}", package)),
            (root_path.join("src/main/resources").join("quote.properties"), QUOTE_PROPERTIES_TEMPLATE.replace("{{package}}", package).to_owned()),
            (root_path.join(".gitignore"), GITIGNORE_TEMPLATE.to_owned()),
            (root_path.join("build.gradle"), QUOTE_BUILD_GRADLE_TEMPLATE.replace("{{group}}", package)),
            (root_path.join("settings.gradle"), SETTINGS_GRADLE_TEMPLATE.replace("{{project_name}}", project_name)),
            (root_path.join("gradle/wrapper/gradle-wrapper.properties"), GRADLE_WRAPPER_PROPERTIES_TEMPLATE.to_owned()),
            (root_path.join("src/main/java").join(package.replace(".", "/")).join("spider/QuoteSpider.java"), QUOTE_SPIDER_TEMPLATE.replace("{{package}}", package)),
            (root_path.join("src/main/java").join(package.replace(".", "/")).join("itempipeline/CsvWriterAuthorItemPipeline.java"), QUOTE_ITEM_PIPELINE_TEMPLATE.replace("{{package}}", package)),
            (root_path.join("src/main/java").join(package.replace(".", "/")).join("item/AuthorItem.java"), QUOTE_ITEM_TEMPLATE.replace("{{package}}", package)),
        ]
    } else {
        vec![
            (root_path.join("src/main/java").join(package.replace(".", "/")).join("Crawler.java"), CRAWLER_JAVA_TEMPLATE.replace("{{package}}", package)),
            (root_path.join("src/main/resources").join(format!("{}.properties", project_name)), APPLICATION_PROPERTIES_TEMPLATE.replace("{{package}}", package).to_owned()),
            (root_path.join(".gitignore"), GITIGNORE_TEMPLATE.to_owned()),
            (root_path.join("build.gradle"), BUILD_GRADLE_TEMPLATE.replace("{{group}}", package)),
            (root_path.join("settings.gradle"), SETTINGS_GRADLE_TEMPLATE.replace("{{project_name}}", project_name)),
            (root_path.join("gradle/wrapper/gradle-wrapper.properties"), GRADLE_WRAPPER_PROPERTIES_TEMPLATE.to_owned()),
        ]
    }
}


fn create_dir(path: &str) {
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("Failed to create project directory {}: {}", path, e);
    }
}

fn create_file_from_template(file_path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
