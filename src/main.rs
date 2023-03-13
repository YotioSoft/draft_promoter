use std::fs;
use std::path::Path;
use std::env;

mod parse;

fn main() {
    // 引数受け取り
    let arg_struct = parse::parser();

    // 既定値
    // writing_posts にいる場合
    let current_dir = env::current_dir().expect("cannot get current dir");
    let (default_from, default_to) = if current_dir.ends_with("writing_posts") {
        (String::from("./"), String::from("../_posts/"))
    }
    // _posts にいる場合
    else {
        (String::from("../writing_posts"), String::from("./"))
    };

    // コピー先ファイル名
    let destination_file = if arg_struct.destination_file.is_empty() {
        arg_struct.source_file.clone()
    }
    else {
        arg_struct.destination_file
    };
    if destination_file.is_empty() {
        println!("source file name is empty.");
        return;
    }

    // ファイルパスの用意
    let from = if arg_struct.from.is_empty() {
        default_from
    }
    else {
        arg_struct.from
    };
    let to = if arg_struct.to.is_empty() {
        default_to
    }
    else {
        arg_struct.to
    };

    let source_file = Path::new(&from).join(&destination_file);
    let destination_file = Path::new(&to).join(&destination_file);

    // ファイルの存在確認
    if !source_file.exists() {
        println!("file {} does not exist.", source_file.display());
        return;
    }
    if !Path::new(&to).exists() {
        println!("directory {} does not exist.", to);
        return;
    }

    // ファイルのコピー
    fs::copy(&source_file, &destination_file).expect("cannot copy file");

    // 表示
    println!("copy {} -> {} done.", source_file.display(), destination_file.display());

    // ソースファイルの削除
    if arg_struct.remove_source {
        fs::remove_file(&source_file).expect("cannot remove file");
        println!("remove {} done.", source_file.display());
    }
}
