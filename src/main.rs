use colored::Colorize;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct ExerciseInfo {
    exercises: Vec<Exercise>,
}

#[derive(Debug, Deserialize)]
struct Exercise {
    name: String,
    path: String,
    #[serde(default)]
    hint: String,
}

fn main() {
    println!("{}", "🌐 Rust Network Protocols - 网络协议学习".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());
    println!();

    // 读取练习配置
    let info_path = Path::new("info.toml");
    if !info_path.exists() {
        eprintln!("{}", "错误: 找不到 info.toml 配置文件".red());
        std::process::exit(1);
    }

    let info_content = fs::read_to_string(info_path).expect("无法读取 info.toml");
    let info: ExerciseInfo = toml::from_str(&info_content).expect("无法解析 info.toml");

    let mut total = 0;
    let mut passed = 0;
    let mut failed_exercises = Vec::new();

    for exercise in &info.exercises {
        total += 1;
        let path = Path::new(&exercise.path);

        if !path.exists() {
            println!("  {} {} (文件不存在)", "⚠".yellow(), exercise.name.yellow());
            continue;
        }

        // 检查文件是否包含 todo!() 或 TODO
        let content = fs::read_to_string(path).unwrap_or_default();
        let has_todo = content.contains("todo!()") || content.contains("// TODO");

        if has_todo {
            println!("  {} {} (待完成)", "○".yellow(), exercise.name);
            failed_exercises.push(exercise);
        } else {
            // 尝试编译和运行测试
            let output = Command::new("cargo")
                .args(["test", "--", "--test", &exercise.name.replace('/', "_")])
                .output();

            match output {
                Ok(result) if result.status.success() => {
                    println!("  {} {}", "✓".green(), exercise.name.green());
                    passed += 1;
                }
                _ => {
                    println!("  {} {} (测试失败)", "✗".red(), exercise.name.red());
                    failed_exercises.push(exercise);
                }
            }
        }
    }

    println!();
    println!("{}", "=".repeat(50).dimmed());
    println!(
        "进度: {}/{} ({}%)",
        passed.to_string().green(),
        total,
        ((passed as f64 / total as f64) * 100.0) as u32
    );

    if !failed_exercises.is_empty() {
        println!();
        println!("{}", "📝 下一个练习:".bold());
        let next = &failed_exercises[0];
        println!("   文件: {}", next.path.cyan());
        if !next.hint.is_empty() {
            println!("   提示: {}", next.hint.dimmed());
        }
        println!();
        println!("   运行测试: cargo test {}", next.name.replace('/', "_"));
    } else {
        println!();
        println!("{}", "🎉 恭喜！你已完成所有练习！".green().bold());
    }
}
