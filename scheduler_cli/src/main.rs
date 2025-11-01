use std::io::{self, Write};


#[derive(Debug, Clone)]
struct Lesson{
    course_name: String,
    teacher: String,
    weekday: String,
    start_time: String,
    end_time: String,
    location: String,
    notes: Option<String>,
}

fn main() {
    let mut lessons: Vec<Lesson> = Vec::new();

    loop{
        println!("请选择操作：——————————————————————————————————————————————————-");
        println!("1. 新增课程");
        println!("2. 删除课程");
        println!("3. 修改课程");
        println!("4. 查看课程");
        println!("5. 退出");
        print!("请输入选项：");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim(){
            "1" => add_lesson(&mut lessons),
            "2" => delete_lesson(&mut lessons),
            "3" => update_lesson(&mut lessons),
            "4" => view_lesson(&lessons),
            "5" => break,
            _ => println!("无效选项，请重新输入"),
        }
    }

}


fn add_lesson(lessons: &mut Vec<Lesson>){
    println!("请输入课程名称：");
    let mut course_name = String::new();
    io::stdin().read_line(&mut course_name).unwrap();
    println!("请输入教师名称：");
    let mut teacher = String::new();
    io::stdin().read_line(&mut teacher).unwrap();
    println!("请输入星期：");
    let mut weekday = String::new();
    io::stdin().read_line(&mut weekday).unwrap();
    println!("请输入开始时间：");
    let mut start_time = String::new();
    io::stdin().read_line(&mut start_time).unwrap();
    println!("请输入结束时间：");
    let mut end_time = String::new();
    io::stdin().read_line(&mut end_time).unwrap();
    println!("请输入上课地点：");
    let mut location = String::new();
    io::stdin().read_line(&mut location).unwrap();
    println!("请输入备注：");
    let mut notes = String::new();
    io::stdin().read_line(&mut notes).unwrap();
    let lesson = Lesson{
        course_name: course_name.trim().to_string(),
        teacher: teacher.trim().to_string(),
        weekday: weekday.trim().to_string(),
        start_time: start_time.trim().to_string(),
        end_time: end_time.trim().to_string(),
        location: location.trim().to_string(),
        notes: Some(notes.trim().to_string()),
    };
    lessons.push(lesson);
    println!("课程添加成功！");
}


fn delete_lesson(_lessons: &mut Vec<Lesson>){
    println!("请输入要删除的课程名称：");
    // TODO: 实现删除逻辑
}

fn update_lesson(_lessons: &mut Vec<Lesson>){
    println!("请输入要修改的课程名称：");
    // TODO: 实现修改逻辑
}

fn view_lesson(lessons: &Vec<Lesson>){
    println!("请输入要查看的课程名称：");
}
