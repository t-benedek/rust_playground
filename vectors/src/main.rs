fn main() {
    let mut v1 = vec![1,2,3,4,5];

    println!("Third element of vector with referencing is {}", &v1[2]);
    println!("Third element of vector with get(2) is {:?}", v1.get(2));

    match &v1.get(2) {
        Some(idx) => println!("Third element of vector in the match statement is {idx}\n"),
        None => println!("No element found on that position"),
    }

    // program will panik here with using &v1
    // println!("100. element of vector is {}", &v1[99]);
    let idx_5 = v1.get(5);
    println!("6th element of vector before push is {:?}", idx_5);
    v1.push(6);
    
    // this wont work here because of borrowing and scope
    // println!("5th element of vector is {:?}", idx_5);
    println!("6th element of vector after push is {:?}", v1.get(5));
    v1[5] = v1[5] + 5;
    println!("6th element of vector after summming is {:?}", v1.get(5));

    println!("\nIterating over a vector with for loop and multiply by *2");
    for elem in &mut v1 {
        *elem *= 2;
        println!("element of vector is {elem}");
    }

    enum Cells {
        Int(i32),
        Text(String),
        Float(f32) 
    }

    let v2: Vec<Cells> = vec![
        Cells::Int(3), 
        Cells::Text(String::from("Hello")), 
        Cells::Float(2.0)
    ];

    println!("\nIterating over a  vector with enums als elements");
    for elem in v2 {
        match elem {
            Cells::Int(i) => println!("element of vector is an Int : {i}"),
            Cells::Float(f) => println!("element of vector is a Float  : {:?}", f),
            Cells::Text(t) => println!("element of vector is a Test  : {t}"),
        }
        
    }


}
