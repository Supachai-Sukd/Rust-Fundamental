// ==========================================
// RUST FUNDAMENTALS FOR GOLANG DEVELOPERS
// ==========================================

fn main() {
    println!("=== RUST FUNDAMENTALS ===");
    
    println!("\n--- 1. Variables & Mutability ---");
    variables_example();

    println!("\n--- 2. Data Types ---");
    data_types_example();

    println!("\n--- 3. Control Flow ---");
    control_flow_example();

    println!("\n--- 4. Ownership & Borrowing (Rust's Magic) ---");
    ownership_example();

    println!("\n--- 5. Structs & Methods ---");
    struct_example();

    println!("\n--- 6. Enums & Pattern Matching ---");
    enum_and_match_example();

    println!("\n--- 7. Error Handling ---");
    error_handling_example();

    println!("\n--- 8. Traits (like Interfaces) ---");
    traits_example();

    println!("\n--- 9. Closures & High-Order Functions ---");
    closures_example();

    println!("\n--- 10. Macros (Metaprogramming) ---");
    macros_example();

    println!("\n--- 11. Lifetimes (The Hardest Syntax) ---");
    lifetimes_example();

    println!("\n--- 12. Common Collections (Vec, HashMap, HashSet) ---");
    collections_example();

    println!("\n--- 13. Path Separator (::) ---");
    path_separator_example();

    println!("\n--- 14. Constants & Statics ---");
    constants_example();
}

// ---------------------------------------------------------
// 1. Variables & Mutability
// ---------------------------------------------------------
// ใน Golang:
// var x int = 5 (mutable เสมอ)
// const y = 10 (immutable)
// x := 5 (shorthand, mutable)
//
// ใน Rust:
// ตัวแปรทุกตัวเป็น immutable (เปลี่ยนค่าไม่ได้) โดย default
// ถ้าจะให้เปลี่ยนค่าได้ต้องใส่ keyword `mut` ไว้ข้างหน้าเวลาประกาศ
fn variables_example() {
    let x = 5; // immutable
    println!("x (immutable) = {}", x);
    // x = 6; // Error! ไม่สามารถเปลี่ยนค่าได้เพราะไม่มีคำว่า `mut`

    let mut y = 10; // mutable
    println!("y (mutable initially) = {}", y);
    y = 15;
    println!("y (after mutation) = {}", y);

    // Rust มีกระบวนการ shadowing (การประกาศตัวแปรชื่อเดิมทับของเดิมใน scope ได้)
    // ใน Golang การใช้ := ประกาศตัวแปรรับค่าตัวแปรเดิมซ้ำใน block เดิมจะพัง (no new variables on left side)
    let x = x + 1;
    println!("x (after shadowing) = {}", x);
}

// ---------------------------------------------------------
// 2. Data Types
// ---------------------------------------------------------
// ใน Golang:
// int, int64, float64, string, bool
//
// ใน Rust:
// i32, i64, f64, String หรือ &str (string slices), bool
fn data_types_example() {
    // Rust มักจะ infer type ให้เราแบบอัตโนมัติ (คล้าย := ใน Go)
    let is_active: bool = true;

    // ข้อควรระวัง: String ใน Rust มี 2 ประเภทหลักที่เจอบ่อยมากๆ:
    // - `&str` (String slice): เทียบเท่า string literal ชี้ไปที่หน่วยความจำ (immutable)
    // - `String`: ตัวที่เป็น Object เก็บใน Heap, สามารถขยาย/แก้ไขได้ (เทียบเท่า string ปกติของ Go)
    let greeting_str: &str = "Hello";
    let mut greeting_string: String = String::from("Hello");
    greeting_string.push_str(" World"); // จัดการ string ใน heap ได้

    println!("Boolean: {}", is_active);
    println!("&str: {}", greeting_str);
    println!("String: {}", greeting_string);

    // Tuple ของ Rust สามารถมี types ต่างกันอยู่ในโครงสร้างได้เลย (Go ไม่มี Tuple ตรงๆ แต่มี multi return types)
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple [0]: {}, [1]: {}", my_tuple.0, my_tuple.1);
}

// ---------------------------------------------------------
// 3. Control Flow
// ---------------------------------------------------------
// ใน Golang:
// if condition { ... }
// for i := 0; i < 5; i++ { ... }
// for condition { ... } // (เอา for มาใช้แทน while)
// switch { ... }
//
// ใน Rust:
// if ไม่ต้องมีวงเล็บเหมือน Go 
// สามารถ return ค่าจาก block if/else ได้ (ใน Go ไม่มี ternary/return แบบนี้ ต้อง assign เอาเอง)
fn control_flow_example() {
    let number = 3;

    // if-else รูปร่างหน้าตาเหมือน Go ครับ
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // ใน Rust if คือ expression มันสามารถสาดค่าออกมารับใส่ตัวแปรได้เลย!
    let is_even = if number % 2 == 0 { true } else { false };
    println!("is number {} even? {}", number, is_even);

    // For loop ของ Rust มักจะวนด้วย Iterator (เหมือน for ... range ใน Go)
    // ส่วน while loop ก็มีให้ใช้ตามปกติ 
    println!("For loop (range):");
    for i in 1..=3 {
        println!("i = {}", i);
    }
}

// ---------------------------------------------------------
// 4. Ownership & Borrowing
// ---------------------------------------------------------
// *** นี่คือหัวใจสำคัญของ Rust ที่ทำให้แตกต่างจากภาษาอื่น
// 
// ใน Golang:
// เรามี Garbage Collector (GC) คอยตามกวาดขยะหน่วยความจำที่ไม่ได้ใช้งานให้
//
// ใน Rust:
// ใช้กฏ Ownership หุ้ม memory ไว้ ไม่มี GC
// 1 data จะมีตัวแปรที่ "เป็นเจ้าของ" ได้แค่ 1 อันเท่านั้น หากเราโยนต่อให้คนอื่น สิทธิ์นั้นจะขาด
// ค่าจะถูก "Move" ยกเว้นเราส่งแบ็คอัพไปเป็น Reference แบบ "ยืม" (Borrow) มาใช้ (`&`)
fn ownership_example() {
    let s1 = String::from("hello_ownership");
    
    // ถ้าเรา let s2 = s1; s1 จะตายทันที เอามา println ต่อจะพัง (Compile ไม่ผ่าน, Ownership ถูกย้ายไปที่ s2 แล้ว)
    
    let s2 = &s1; // ยืมแค่อ่าน (เหมือนเราส่ง Pointer &s1 ใน Go ชั่วคราว)

    // ใน Go pointer ใช้แก้ค่าหรือแค่ reference ตรงนี้ก็เหมือนกัน ต่างที่ Rust เช็คจุกจิกเรื่องยืมให้ปลอดภัยมากๆตอน compile
    println!("s1 remains active: {}", s1);
    println!("s2 borrowed from s1: {}", s2);
}

// ---------------------------------------------------------
// 5. Structs & Methods
// ---------------------------------------------------------
// ใน Golang:
// type User struct { Name string }
// func (u *User) GetName() { ... }
//
// ใน Rust:
// รูปร่าง Data Structure คล้ายกัน คือประกาศ struct แล้วเอา method ไปแปะใน block `impl`
struct User {
    username: String,
    active: bool,
}

// implementation logic ทั้งหมดรวมกันอยู่ใน block เดียว ไม่กระจายเหมือนของ Go
impl User {
    // ฟังก์ชันที่รับ &self แปลว่าเป็น Method (ใน Go คือตัว receiver type (u *User))
    fn is_active(&self) -> bool {
        self.active
    }

    // ฟังก์ชันที่ไม่มี &self เรียกว่า Associated Function 
    // เหมือนเวลาทำ Constructor ของ Go (เช้น func NewUser() *User)
    fn new(username: &str) -> User {
        User {
            username: String::from(username),
            active: true,
        }
    }
}

fn struct_example() {
    let user1 = User::new("gopher_learning");
    println!("User: {}, Active?: {}", user1.username, user1.is_active());
}

// ---------------------------------------------------------
// 6. Enums & Pattern Matching
// ---------------------------------------------------------
// ใน Golang:
// ไม่มี Enum ตรงๆ ต้องประยุกต์ const กับ iota แล้วใช้ switch ในการเช็คแต่ละแบบเอาเอง
//
// ใน Rust:
// Enum ของจริง!! แถมทรงพลังที่สุด มันสามารถเก็บข้อมูลลงไปในบางหัวข้อ (Variant) ของ Enum ได้เลย
enum WebEvent {
    PageLoad,
    KeyPress(char),            // มีข้อมูล char แนบไว้ใน state
    Click { x: i64, y: i64 },  // แนบข้อมูล struct พิกัดเม้าส์ได้เลย!
}

fn enum_and_match_example() {
    let event = WebEvent::KeyPress('R');

    // match ของ Rust บังคับต้องดักให้ครบทุกหน้า (Exhaustive) และดึงข้อมูลที่แฝงอยู่ออกมาได้เลย
    // ของ Go switch/case จะมีโอกาสเขียนหลุดได้ไม่ครบเคส หรือถ้าแฝงข้อมูลก็ต้องทำ Struct Interface เล่นแร่แปรธาตุกันเหนื่อย
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::KeyPress(c) => println!("Key '{}' pressed", c),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
    }
}

// ---------------------------------------------------------
// 7. Error Handling
// ---------------------------------------------------------
// ใน Golang:
// เราถนัดมาก: func DoJob() (string, error)
// และชินชากับการ: if err != nil { return nil, err }
//
// ใน Rust:
// ไม่มีการ return สองตัว (Tuple return pattern ของ Go)
// ให้ค่า Result<T, E> มาเป็น Standard แทน ซึ่งก็คือ Enum ของข้อ 6 (มี 2 อาการคือ Ok(value) / Err(value))
fn read_file_mock() -> Result<String, String> {
    let success = true; // ลองปรับเป็น false จะเห็นภาพ
    
    if success {
        Ok(String::from("data from file"))
    } else {
        Err(String::from("file not found"))
    }
}

fn perform_action() -> Result<(), String> {
    // ใน Rust จะมีเครื่องหมายปริศนา `?` (Question Mark Operator) ต่อท้ายคำสั่งที่พ่น Result
    // การใส่ ? = "เห้ย ถ้าเป็น error (Err) ให้ return ออกจากฟังก์ชันนี้ส่ง error ไปให้คนเรียกเลย"
    // เทียบเท่า if err != nil { return err } ใน Go เดี๊ยะๆ
    let data = read_file_mock()?; 
    println!("Read content was a success: {}", data);
    Ok(()) // () ของ Rust หมายถึง return empty แบบ void (เทียบเท่า error == nil ใน Go)
}

fn error_handling_example() {
    match perform_action() {
        Ok(_) => println!("Action completed without problems."),
        Err(e) => println!("Boom! Error occurred: {}", e),
    }
}

// ---------------------------------------------------------
// 8. Traits (like Interfaces)
// ---------------------------------------------------------
// ใน Golang:
// Duck Typing! เราใช้ type Speaker interface { Speak() string }
// แล้วมี Struct ที่สร้างรับ Speak() เป็นอันถือว่าผ่าน interface อัตโนมัติ (Implicit)
//
// ใน Rust:
// Trait = Interface
// ต้องประกาศแบบจะแจ้งว่า Struct นี้ Implement โลจิกนี้ (Explicit)
trait Speaker {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

// ประกาศเจาะจงลงไปเลยว่าขอ implement trait นี้ให้หมา
impl Speaker for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

fn traits_example() {
    let dog = Dog { name: String::from("Buddy") };
    
    // เวลาทำ polymorphism ส่ง interface ใน Golang: func makeNoise(s Speaker)
    // ใน Rust: คือการส่ง parameter รับ &impl Trait ต่างๆ
    fn make_noise(s: &impl Speaker) {
        println!("{}", s.speak());
    }

    make_noise(&dog);
}

// ---------------------------------------------------------
// 9. Closures (Anonymous Functions)
// ---------------------------------------------------------
// ใน Golang:
// เราสามารถเขียน Anonymous function ได้เลยเช่น: add := func(a, b int) int { return a + b }
//
// ใน Rust:
// เรียกว่า Closure ใช้สัญลักษณ์ท่อ `| |` ครอบตัวแปรที่รับเข้ามา
fn closures_example() {
    // Closure แบบสั้นๆ (Rust จะเดา Type ให้เองได้)
    let add_one = |x| x + 1;
    println!("Closure execution: 5 + 1 = {}", add_one(5));

    // จุดที่ใช้บ่อยคือ Iterator methods (เหมือน Map/Filter/Reduce ของภาษาอื่น ซึ่ง Go ไม่มีให้ใช้ตรงๆ)
    let nums = vec![1, 2, 3, 4, 5]; // vec! คือ Macro สำหรับสร้าง Vector (Array แบบยืดหยุ่นได้เหมือน Slice)
    
    // .into_iter() = แปลงเป็น Iterator ก่อน
    // .filter() = ใช้ Closure คัดกรองตัวเลขออกมาเฉพาะที่ % 2 ลงตัว (เลขคู่)
    // .collect() = ประกอบกลับร่างกลายเป็น Vector โนยต้องระบุ Type ไว้ตัวแปรที่รับค่า (หรือเขียนท่า turbofish ก็ได้)
    let evens: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 0).collect();
    
    // ใช้ {:?} ในการปริ้นท์โครงสร้างของ Vector หรือ Struct ทั่วไป
    println!("Filtered even numbers: {:?}", evens); 
}

// ---------------------------------------------------------
// 10. Macros
// ---------------------------------------------------------
// ใน Golang:
// โค้ดที่แพทเทิร์นซ้ำหรืองาน Code Generation ต้องใช้ tool ภายนอกสั่งรันเช่น `go generate`
// 
// ใน Rust:
// มีระบบ Macro (การเขียนโค้ดที่สคริปต์ไปเสกโค้ดขึ้นมาอีกที) ที่ทรงพลังและปลอดภัยมาก 
// จุดสังเกตของ Macro คือจะมีสัญลักษณ์ `!` ต่อท้ายชื่อ เช่น println!, vec!, format!
// สามารถสร้าง Macro แจ่มๆ เพื่อลดความซ้ำซ้อนของการเขียนโค้ดถึกๆได้
macro_rules! say_hello_many_times {
    // กำหนดรูปแบบ (Pattern) ที่จะรับเข้ามา `$name` คือระบุชื่อตัวแปร และ `:expr` คือบอกว่าเป็นนิพจน์ทั่วๆไป (Expression)
    ($name:expr, $times:expr) => {
        // block นี้จะถูก "คัดลอก" เอาไปแปะแทนที่จุดพิมตอน compile (Zero-cost จริงๆ)
        for _ in 0..$times {
            println!("Hello macro, {}!", $name);
        }
    };
}

fn macros_example() {
    // เรียกใช้ Macro ที่เราสร้างเอง โค้ดตรงบรรทัดนี้จะขยายตัวแตกออกกลายเป็น for loop ให้เองตอนรันคำสั่ง Compile
    say_hello_many_times!("Gopher", 2);
}

// ---------------------------------------------------------
// 11. Lifetimes (Syntax ปราบเซียนที่ทำให้หลายคนท้อ)
// ---------------------------------------------------------
// ใน Golang:
// ส่ง Pointer ไปใช้เถอะเดี๋ยว Garbage Collector จัดการให้ โค้ดไม่มีทางพังเพราะ Pointer ชี้เศษซากแน่ๆ (แต่อาจจะช้าตอน GC หนักๆ)
//
// ใน Rust:
// ปัญหาของระบบที่ไม่มี GC คือ "ถ้ายืม (Borrow) ข้อมูลมา แล้วเจ้าของตัวจริงตายไปก่อนขืนเรียกใช้ก็พังดิ!" (เรียกว่า Dangling Pointer)
// Rust เลยมีระบบ Lifetime ระบุ "อายุขัย" ของ Reference (pointer แบบยืม) โดยใช้สัญลักษณ์ `'` (single quote) นำหน้า
// (เช่น `'a` อ่านว่า lifetime "a")
// ส่วนใหญ่ Compile จะแอบเดาให้เราแบบเงียบๆ แต่บางทีถ้ามันโยงมั่วซั่ว มันจะบังคับหาคนประทับตรารับพิจารณาเอง!

// ตัวอย่าง: ถ้าเขียนแบบนี้ `fn longest(x: &str, y: &str) -> &str` (ไม่มี 'a) คอมไพล์จะพังเพราะมันไม่รู้ว่าตัวที่ Return จะอายุเท่าตัวไหน
// แปลภาษาคนเรื่องการแทรก 'a ขี้เหร่ๆเหล่านี้คือ: 
// "รับ Reference อย่างน้อย 2 ตัว (x และ y) ที่อย่างน้อยที่สุดต้องมีชีวิตรอดเท่ากับระยะเวลาของ 'a"
// "และรับประกันว่าค่าสุดท้ายที่พ่น return ทิ้งไว้ให้นั้น จะมีชีวิตอยู่รอดได้ตามกรอบเวลา 'a (หรือเท่ากับตัวแปรที่ตายไวกว่าในสองตัวในที่ส่งมา)"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetimes_example() {
    let string1 = String::from("long_string_owner_forever");
    let result;
    
    // เปิด Block จำลองพื้นที่เกิด-ตาย ของตัวแปร
    {
        let string2 = "short"; // สมมุติ string2 โดนสร้างใน scope แคบๆ 
        
        // ทั้ง string1 และ string2 ถูกโยน Reference ขึ้นเขียงให้ longest พิจารณา
        // Rust จะเช็ค Lifetime ให้ตอน Compile ถ้าผ่านคือวิ่งฉลุย (เร็วทะลุนรก แถมไร้ Overhead)
        result = longest(string1.as_str(), string2);
        println!("The longest string is: {}", result);
    }
    
    // คำเตือน ⚠️
    // ถ้าเราเอา `println!("{}", result)` มาวางตรงบรรทัดนี้ข้างนอก block...
    // โค้ดจะด่าหยาบและพังทันทีตอนสั่ง compile! 
    // เพราะ Rust จำไว้แล้วว่าผลลัพธ์จาก longest ของเรา ถูกผูกติด 'a กับข้อมูลของ string2 ด้วย... 
    // และตรงบรรทัดนี้ string2 ได้ตายไปจากโลกนี้ (โดน Drop) เรียบร้อยแล้ว!
}

// ---------------------------------------------------------
// 12. Common Collections (Data Structures หลัก)
// ---------------------------------------------------------
// ใน Golang:
// เรามี Array, Slice (`[]int`), และ Map (`map[string]int`) บิวท์อินมากับภาษาเลย
// 
// ใน Rust:
// จะมีชุดเครื่องมือจัดเก็บข้อมูล (Collections) ที่เก็บอยู่ใน heap ให้เลือกใช้อยู่ใน Standard Library (`std::collections`)
use std::collections::{HashMap, HashSet};

fn collections_example() {
    // 1. Vector (Vec<T>) 
    // พฤติกรรมเหมือน Slice ของ Golang คือ Array ที่ขยายขนาดตัวเองได้ (Growable Array)
    let mut my_vec: Vec<i32> = Vec::new(); // แบบสร้างใหม่เปล่าๆ
    my_vec.push(10);
    my_vec.push(20);
    
    // แบบย่อโดยใช้ Macro `vec!` (นิยมกว่า)
    let mut scores = vec![100, 95, 80];
    scores.push(75);
    
    println!("Vector: {:?}", scores);
    
    // ** การดึงค่าออกมา (Access) **
    // ของ Go ถ้าดึง index ล้น เช่น slice[10] ตัวโปรแกรมจะ Panic (runtime error) แตกลงตรงนั้นเลย
    // ของ Rust ก็พังเหมือนกันถ้าเราดึงตรงๆว่า `scores[10]` 
    // !! แต่ Rust มีท่า `.get()` ที่มันจะแอบเอา Type Option (Some/None) มาคลุมให้ ถ้าไม่เจอแค่พ่น None ออกมา โปรแกรมไม่ค้าง
    match scores.get(10) {
        Some(s) => println!("Found score: {}", s),
        None => println!("Score index 10 not found. Safe! Program won't panic."),
    }

    // 2. HashMap<K, V> (เหมือน map[K]V ใน Go)
    // เก็บข้อมูลแบบ Key-Value
    let mut user_roles = HashMap::new();
    user_roles.insert(String::from("Alice"), "Admin");
    user_roles.insert(String::from("Bob"), "User");
    
    println!("HashMap: {:?}", user_roles);

    // ** เช็คก่อนอัปเดต หรือใส่ค่าเริ่มต้นถ้า Key นี้ดร็อปหายไป **
    // ของ Go ต้องเขียน: if _, ok := roles["Charlie"]; !ok { roles["Charlie"] = "User" }
    // ของ Rust: มีสิ่งที่เรียกว่า Entry API มาให้ โคตรหล่อและสั้น
    user_roles.entry(String::from("Charlie")).or_insert("User"); // เนื่องจาก Charlie ยังไม่มี เลยถูกยัด "User" เข้าไป
    user_roles.entry(String::from("Alice")).or_insert("SuperAdmin"); // Alice เป็น Admin อยู่แล้ว ท่อนนี้เลยจะไม่ทำงาน

    println!("Alice's role: {}", user_roles.get("Alice").unwrap()); // .unwrap() เป็นการบอกให้แกะ Option ออกมาเลย (มั่นใจว่ามีชัวร์ๆ)

    // 3. HashSet<T> (เหมือน Set)
    // Golang ไม่มี Type Set แบบทางการ มักจะต้อง Hack ใช้ `map[string]bool` หรือ `map[string]struct{}` 
    // แต่ใน Rust มีมาให้เลย เอาไว้เก็บค่าที่ไม่ซ้ำกัน ค้นหาได้เร็ว
    let mut unique_ids = HashSet::new();
    unique_ids.insert(101);
    unique_ids.insert(102);
    unique_ids.insert(101); // เสียบ 101 ซ้ำ ค่าจะถูกโยนทิ้งอัตโนมัติ

    println!("HashSet (unique elements only): {:?}", unique_ids);
}

// ---------------------------------------------------------
// 13. Path Separator (::) คืออะไร
// ---------------------------------------------------------
// ใน Golang:
// เราใช้เครื่องหมายจุด `.` (Dot) ในการทำแทบจะทุกอย่าง 
// ทั้งเข้าถึง Method, ตัวแปรโครงสร้าง Struct, และแม้กระทั่งการเรียกข้าม Package เช่น `fmt.Println()`, `strings.ToLower()`
//
// ใน Rust:
// จะแบ่งแยกหน้าที่ของสัญลักษณ์ออกจากกันอย่างชัดเจนมาก:
// 1. เครื่องหมายจุด `.` : ใช้เรียกข้ามผ่าน "ตัวของมันเอง (Instance objects)" (เช่น my_string.len(), user.name)
// 2. เครื่องหมายคอลอนคู่ `::` (Path Separator) : ใช้เรียกเจาะทะลุเข้าไปใน "โครงสร้างใหญ่" เช่น Module, Static Method/Associated Function, หรือ Enum

// ตัวอย่างจำลองการทำ Module ภายใน (ให้อารมณ์เหมือน Package ใน Go)
pub mod my_math {
    // สังเกตว่าใน Rust ถ้าจะให้คนนอกเรียกได้ ต้องใส่ คำว่า `pub` (พฤติกรรมเดียวกับการตั้งชื่อขึ้นต้นด้วยตัวพิมพ์ใหญ่แบบ Go)
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

fn path_separator_example() {
    // กรณีที่ 1: เรียกใช้ฟังก์ชันที่อยู่ใน Module (Namespace)
    // ของ Go: `my_math.Add(10, 5)`
    // ของ Rust: เจาะเข้าไปด้วย `::`
    let sum = my_math::add(10, 5);
    println!("Calling module function: {}", sum);

    // กรณีที่ 2: เรียก Associated Function (พฤติกรรมคล้าย Static Method)
    // สังเกตว่าเราไม่ได้เรียกผ่านตัวแปรที่เกิดมาแล้ว แต่เรามุดไปที่แปลน "ชนิดข้อมูล (Type)" คือก้อน `String` โดยตรงเพื่อสั่งสร้างมันขึ้นมา
    let s1 = String::from("Hey there"); 
    let my_vec: Vec<i32> = Vec::new(); // Vec::new ก็ใช่!
    println!("Allocating string with String::from(): {:?}", s1);

    // กรณีที่ 3: ดึงประเภทตัวเลือก (Enum Variants)
    // ท่าเต็มของมันคือ `std::result::Result::Ok("Success")` หรือ `std::option::Option::Some(10)`
    // แต่ส่วนนี้ Rust ใจดี แอบเอา Enum สำคัญๆอย่าง Ok, Err, Some, None มาดักรอใน scope ไว้ให้แล้ว เลยไม่ต้องพิมพ์ namespace ยาวๆ
    
    // ** บทสรุปการจำง่ายๆ สำหรับสาย Go: **
    // - เจอ `::` เมื่อไหร่ ให้รู้ไว้ว่า คือการเรียก "ชื่อครอบครัว / Namespace / ชื่อโมดูลใหญ่ๆ"
    // - เจอ `.` เมื่อไหร่ ให้รู้ไว้ว่า คือการเรียกใช้งาน "พฤติกรรมผ่านสิ่งของ (Instance) ชิ้นนั้นๆที่ถูกสร้างมาแล้ว"
}

// ---------------------------------------------------------
// 14. Constants & Statics (ตัวแปรคงที่ และ ตัวแปรระดับ Global)
// ---------------------------------------------------------
// ใน Golang:
// const MAX_USERS = 100
// var GlobalCounter int = 0 (เป็น global variable ได้ ถ้าอยู่นอกสุด)
//
// ใน Rust:
// จะมีตัวแปรที่ใช้นอกฟังก์ชันได้ 2 แบบคือ `const` และ `static`
// 1. `const`: ค่าคงที่แท้จริง ไม่มีการจอง Memory เป็นชิ้นเป็นอัน ค่าจะถูก Inlined แทรกตรงๆ ไปตัวรันโปรแกรมเลย บังคับบอก Type ชัดเจน (เหมือน const ฝั่ง Go)
// 2. `static`: ข้อมูลที่มีตำแหน่ง Address ใน Memory จริงๆ (Global Variable มีอายุยืนเท่าตอนรัน) ปกติก็ห้ามเปลี่ยนค่าเหมือนกัน
//    - กรณีอยากเปลี่ยนค่าจริงๆ ต้องใช้ท่า `static mut` และการไปยุ่งกับมันจะถูกบีบให้กระทำภายใต้บล็อก `unsafe { ... }` 
//    - เพื่อเตือนสติคนเขียนว่า Rust ไม่รับประกันความปลอดภัยตอน Multi-thread นะโว้ย ลุยเอาเอง!

// สังเกตว่าทั้งคู่มักตั้งชื่อแบบ SCREAMING_SNAKE_CASE
const MAX_PLAYERS: u32 = 100;
static mut GLOBAL_VISITS: i32 = 0; // ควรหลีกเลี่ยงถ้าไม่จำเป็นจริงๆ

fn constants_example() {
    println!("Max players allowed (const): {}", MAX_PLAYERS);

    // MAX_PLAYERS = 200; // ท่านี้จะโดนด่าแต่ไกลเลยว่า "invalid left-hand side of assignment"

    // การเข้าถึงหรือแก้ไขตัวแปร static mut ต้องทำใน block unsafe เท่านั้น
    unsafe {
        GLOBAL_VISITS += 1;
        println!("We have a visitor! Total visits (static mut): {}", GLOBAL_VISITS);
    }
}
