// Q1.1)
// use std::f64::consts::PI;

// #[derive(Debug, Clone, Copy)]
// enum Shape {
//     Circle(i32, i32, i32),
//     Rectangle(i32, i32, i32, i32),
//     Triangle(i32, i32, i32, i32, i32, i32),
// }

// impl Shape {
//     fn rep_string(&self) -> String {
//         match self {
//             Shape::Circle(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
//             Shape::Rectangle(x, y, w, h) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
//             Shape::Triangle(x1, y1, x2, y2, x3, y3) => format!(
//                 "<Triangle: {}, {}, {}, {}, {}, {}>",
//                 x1, y1, x2, y2, x3, y3
//             ),
//         }
//     }

//     fn area(&self) -> f64 {
//         match self {
//             Shape::Circle(_, _, r) => PI * (*r * *r) as f64,
//             Shape::Rectangle(_, _, w, h) => (*w * *h) as f64,
//             Shape::Triangle(x1, y1, x2, y2, x3, y3) => {
//                 0.5 * ((x1 - x3) * (y2 - y1) - (x1 - x2) * (y3 - y1)) as f64
//             }
//         }
//     }
// }

// fn main() {
//     let shapes = [
//         Shape::Circle(0, 0, 1),
//         Shape::Circle(50, 50, 15),
//         Shape::Rectangle(40, 40, 20, 20),
//         Shape::Rectangle(10, 40, 15, 10),
//         Shape::Triangle(0, 0, 1, 3, 3, 1),
//     ];

//     for shape in shapes.iter() {
//         println!("{}, area: {:.2}", shape.rep_string(), shape.area());
//     }
// }


// Q1.2)
// use std::f64::consts::PI;

// trait Shape: ShapeClone {
//     fn rep_string(&self) -> String;
//     fn area(&self) -> f64;
// }

// #[derive(Debug, Clone, Copy)]
// enum ShapeEnum {
//     Circle(i32, i32, i32),
//     Rectangle(i32, i32, i32, i32),
//     Triangle(i32, i32, i32, i32, i32, i32),
// }

// impl ShapeEnum {
//     fn rep_string(&self) -> String {
//         match self {
//             ShapeEnum::Circle(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
//             ShapeEnum::Rectangle(x, y, w, h) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
//             ShapeEnum::Triangle(x1, y1, x2, y2, x3, y3) => format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1, y1, x2, y2, x3, y3),
//         }
//     }

//     fn area(&self) -> f64 {
//         match self {
//             ShapeEnum::Circle(_, _, r) => PI * (*r * *r) as f64,
//             ShapeEnum::Rectangle(_, _, w, h) => (*w * *h) as f64,
//             ShapeEnum::Triangle(x1, y1, x2, y2, x3, y3) => 0.5 * f64::abs(((x1 - x3) * (y2 - y1) - (x1 - x2) * (y3 - y1)) as f64),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct Circle {
//     x: i32,
//     y: i32,
//     r: i32,
// }

// #[derive(Debug, Clone)]
// struct Rectangle {
//     x: i32,
//     y: i32,
//     w: i32,
//     h: i32,
// }

// #[derive(Debug, Clone)]
// struct Triangle {
//     x1: i32,
//     y1: i32,
//     x2: i32,
//     y2: i32,
//     x3: i32,
//     y3: i32,
// }

// impl Shape for Circle {
//     fn rep_string(&self) -> String {
//         format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
//     }

//     fn area(&self) -> f64 {
//         PI * (self.r * self.r) as f64
//     }
// }

// impl Shape for Rectangle {
//     fn rep_string(&self) -> String {
//         format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h)
//     }

//     fn area(&self) -> f64 {
//         (self.w * self.h) as f64
//     }
// }

// impl Shape for Triangle {
//     fn rep_string(&self) -> String {
//         format!("<Triangle: {}, {}, {}, {}, {}, {}>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
//     }

//     fn area(&self) -> f64 {
//         0.5 * f64::abs(((self.x1 - self.x3) * (self.y2 - self.y1) - (self.x1 - self.x2) * (self.y3 - self.y1)) as f64)
//     }
// }

// impl Circle {
//     fn new(x: i32, y: i32, r: i32) -> Box<dyn Shape> {
//         Box::new(Circle { x, y, r })
//     }
// }

// impl Rectangle {
//     fn new(x: i32, y: i32, w: i32, h: i32) -> Box<dyn Shape> {
//         Box::new(Rectangle { x, y, w, h })
//     }
// }

// impl Triangle {
//     fn new(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Box<dyn Shape> {
//         Box::new(Triangle { x1, y1, x2, y2, x3, y3 })
//     }
// }

// fn input_shape_list() -> Vec<Box<dyn Shape>> {
//     vec![
//         Circle::new(0, 0, 1),
//         Circle::new(50, 50, 15),
//         Rectangle::new(40, 40, 20, 20),
//         Rectangle::new(10, 40, 15, 10),
//         Triangle::new(0, 0, 1, 1, 2, 0),
//     ]
// }

// trait ShapeClone {
//     fn clone_box(&self) -> Box<dyn Shape>;
// }

// impl Clone for Box<dyn Shape> {
//     fn clone(&self) -> Box<dyn Shape> {
//         self.clone_box()
//     }
// }

// impl<T> ShapeClone for T
// where
//     T: 'static + Shape + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Shape> {
//         Box::new(self.clone())
//     }
// }

// fn main() {
//     let shape_list = input_shape_list();
//     let omap = shape_list.iter().map(|s| s.rep_string());
//     let output: Vec<_> = omap.collect();
//     println!("{:?}", output);

//     let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
//     let output: Vec<_> = omap.collect();
//     println!("{:?}", output);

//     let input_list = input_shape_list();
//     let shape_list = input_list.iter().cloned().map(|s| s.clone_box());
//     let omap = shape_list.map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
//     let output: Vec<_> = omap.collect();
//     println!("{:?}", output);
// }



// Q2.1),Q2.2)
// #[derive(Clone)]
// enum Text {
//     Plain(String),
//     Repeated(Box<Text>, usize),
//     Joined(Vec<Box<Text>>, Box<Text>),
// }

// impl Text {
//     fn value(&self) -> String {
//         match self {
//             Text::Plain(t) => t.clone(),
//             Text::Repeated(text, count) => {
//                 let repeated_text = text.value();
//                 repeated_text.repeat(*count)
//             }
//             Text::Joined(texts, separator) => {
//                 let text_values: Vec<String> = texts.iter().map(|t| t.value()).collect();
//                 text_values.join(&separator.value())
//             }
//         }
//     }
// }

// impl From<&Text> for Box<Text> {
//     fn from(t: &Text) -> Box<Text> {
//         Box::new(t.clone())
//     }
// }

// impl AsRef<Text> for Text {
//     fn as_ref(&self) -> &Text {
//         &self
//     }
// }

// trait TextTrait {
//     fn value(&self) -> String;
//     fn clone_box(&self) -> Box<dyn TextTrait>;
// }

// impl Clone for Box<dyn TextTrait> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

// #[derive(Clone)]
// struct PlainText {
//     chars: String,
// }

// impl From<&str> for PlainText {
//     fn from(text: &str) -> PlainText {
//         PlainText {
//             chars: text.to_string(),
//         }
//     }
// }

// impl TextTrait for PlainText {
//     fn value(&self) -> String {
//         self.chars.clone()
//     }

//     fn clone_box(&self) -> Box<dyn TextTrait> {
//         Box::new(self.clone())
//     }
// }

// impl AsRef<dyn TextTrait> for PlainText {
//     fn as_ref(&self) -> &(dyn TextTrait + 'static) {
//         self
//     }
// }

// #[derive(Clone)]
// struct RepeatedText {
//     text: Box<dyn TextTrait>,
//     count: usize,
// }

// impl RepeatedText {
//     fn with_parts(text: &dyn TextTrait, count: usize) -> RepeatedText {
//         RepeatedText {
//             text: text.clone_box(),
//             count,
//         }
//     }
// }

// impl TextTrait for RepeatedText {
//     fn value(&self) -> String {
//         let repeated_text = self.text.value();
//         repeated_text.repeat(self.count)
//     }

//     fn clone_box(&self) -> Box<dyn TextTrait> {
//         Box::new(self.clone())
//     }
// }

// impl AsRef<dyn TextTrait> for RepeatedText {
//     fn as_ref(&self) -> &(dyn TextTrait + 'static) {
//         self
//     }
// }

// #[derive(Clone)]
// struct JoinedText {
//     texts: Vec<Box<dyn TextTrait>>,
//     separator: Box<dyn TextTrait>,
// }

// impl JoinedText {
//     fn with_parts(texts: &[Box<dyn TextTrait>], separator: &dyn TextTrait) -> JoinedText {
//         JoinedText {
//             texts: texts.iter().map(|t| t.clone_box()).collect(),
//             separator: separator.clone_box(),
//         }
//     }
// }

// impl TextTrait for JoinedText {
//     fn value(&self) -> String {
//         let text_values: Vec<String> = self.texts.iter().map(|t| t.value()).collect();
//         text_values.join(&self.separator.value())
//     }

//     fn clone_box(&self) -> Box<dyn TextTrait> {
//         Box::new(self.clone())
//     }
// }

// impl AsRef<dyn TextTrait> for JoinedText {
//     fn as_ref(&self) -> &(dyn TextTrait + 'static) {
//         self
//     }
// }

// #[test]
// fn test_text_repeated() {
//     let t1 = PlainText::from("Hi");
//     let t2 = PlainText::from("[+]");
//     let t3 = RepeatedText::with_parts(&t2, 3);
//     let t4 = RepeatedText::with_parts(&t3, 5);

//     assert_eq!(t1.value(), "Hi");
//     assert_eq!(t2.value(), "[+]");
//     assert_eq!(t3.value(), "[+]".repeat(3));
//     assert_eq!(t4.value(), "[+]".repeat(15));
// }

// #[test]
// fn test_tex_repeated() {
//     let t1 = PlainText::from("Hi");
//     let t2 = PlainText::from("[+]");
//     let t3 = RepeatedText::with_parts(&t2, 3);
//     let t4 = RepeatedText::with_parts(&t3, 5);

//     assert_eq!(t1.value(), "Hi");
//     assert_eq!(t2.value(), "[+]");
//     assert_eq!(t3.value(), "[+]".repeat(3));
//     assert_eq!(t4.value(), "[+]".repeat(15));
// }

// fn main() {
//     test_text_repeated();
//     test_tex_repeated();
// }
