fn main() {
    
    question_1();
    question_2();
    question_3();
    question_4();
    question_5();
    question_6();

}

#[derive(Debug)]
enum Student {

    Online,
    Onsite,

}

fn question_1() {
    
    let option_1 = Student::Online;
    let option_2 = Student::Onsite;
    
    println!("\n-------------");
    println!("Question # 1");
    println!("-------------\n");

    println!("{:?}", option_1);
    println!("{:?}", option_2);

}


#[derive(Debug)]
struct vehicle_types {

    Cars: String,
    Trucks: String,
    Bikes: String,

}

fn question_2() {

    let vehicles = vehicle_types {
        Cars: String::from("Honda Life"),
        Trucks: String::from("ISUZU Truck"),
        Bikes: String::from("Homda CD 70"),

    };

    println!("\n-------------");
    println!("Question # 2");
    println!("-------------\n");

    println!("{:#?}\n", vehicles);

}

#[derive(Debug)]
enum Vehicles {
    
    Cars(String),
    Trucks(String),
    Bikes(String),

}

fn question_3() {

    let cars = Vehicles::Cars(String::from("Honda Life"));
    let trucks = Vehicles::Cars(String::from("ISUZU Truck"));
    let bikes = Vehicles::Bikes(String::from("Honda CD 70"));

    println!("\n-------------");
    println!("Question # 3");
    println!("-------------\n");

    println!("{:?}", cars);
    println!("{:?}", trucks);
    println!("{:?}\n", bikes);

}

#[derive(Debug)]
enum Shape {

    Circle,
    Triangle,
    Rectangle,
    Square,

}

fn shape_values(shape: Shape) -> u8 {

    match shape {
        
        Shape::Circle => 10,
        Shape::Triangle => 180,
        Shape::Rectangle => 160,
        Shape::Square => 220,

    }
}

fn question_4() {
    
    let circle = shape_values(Shape::Circle);
    let triangle = shape_values(Shape::Triangle);
    let rectangle = shape_values(Shape::Rectangle);
    let square = shape_values(Shape::Square);

    println!("\n-------------");
    println!("Question # 4");
    println!("-------------\n");

    println!("Radius of Circle is : {:?}", circle);
    println!("Length of sides of Trainagle is : {:?}", triangle);
    println!("Length of sides of Rectangle is : {:?}", rectangle);
    println!("Length of sides of Square is : {:?}\n", square);

}

fn question_5() {

    let one_fifty = Some(150);
    let six_hundred_fourteen = Some(614.98);
    let some_string = Some("How are you ?");
    let no_number: Option<f32> = None;

    println!("\n-------------");
    println!("Question # 5");
    println!("-------------\n");

    println!("{:?}", one_fifty);
    println!("{:?}", six_hundred_fourteen);
    println!("{:?}", some_string);
    println!("{:?}\n", no_number); 

}

#[derive(Debug)]
enum Laptops {

    HP,
    Dell(Models),
    Asus,
    Lenovo,

}

#[derive(Debug)]
enum Models {

    Series_1000,
    Series_2000,
    Series_3000,
    Series_4000,
    Series_5000,
    Series_6000,

}

fn check_model(laptops: Laptops) {
    
    match laptops {
        
        HP => {
            println!("Hewllett Packard");
        },
      
        Dell => {
            println!("Dell Computers");
        },
        
        Asus => {
            println!("ASUS");
        },

        Lenovo => {
            println!("IBM Lenovo");
        },

    }
    
}

fn question_6() {

    println!("\n-------------");
    println!("Question # 6");
    println!("-------------\n");
    
    check_model(Laptops::HP);
    check_model(Laptops::Asus);
    check_model(Laptops::Lenovo);
    // check_model(Laptops::Dell(Models::Series_1000));
    // check_model(Laptops::Dell(Models::Series_2000));
    // check_model(Laptops::Dell(Models::Series_3000));
    // check_model(Laptops::Dell(Models::Series_4000));
    // check_model(Laptops::Dell(Models::Series_5000));
    // check_model(Laptops::Dell(Models::Series_6000));

}

