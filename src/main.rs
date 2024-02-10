mod random_number_question;
mod pig_latin_game;

use std::io;

fn main() {
    loop {
        println!("変換タイプを選択してください。");
        println!("1: ランダムな整数リストから平均値, 真ん中の値, 最も頻出する値を求める");
        println!("2: 文字列をピック・ラテン");
        println!("3: ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイス");
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("読み込みにに失敗しました。");

        match choice.trim() {
            "1" => random_number_question::output(),
            "2" => pig_latin_game::pig_latin_exec(),
            "3" => continue,
            _ => continue,
        }
        break;
    }
}
