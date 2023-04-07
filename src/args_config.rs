pub struct Config {
    pub city: String,
    pub country: String,
    pub region: String,
}

//I saw that the api used has great minor precision
//if "turkey ankara ankara" is written the api does not accept the request
#[allow(dead_code)]
impl Config {
    pub fn new(args: Vec<String>) -> Config {
        if args.len() < 2 {
            print!("Usage: {} [country] [region] [city]", args[0]);
        }

        //if the entered word is uppercase or lowercase, it is changed to all lowercase letters.
        let country = args[1].clone().to_lowercase();
        let region = args[2].clone().to_lowercase();
        let city = args[3].clone().to_lowercase();

        Config {
            city: city[0..1].to_ascii_uppercase() + &city[1..],
            country: country[0..1].to_ascii_uppercase() + &country[1..],
            region: region[0..1].to_ascii_uppercase() + &region[1..],
        }
    }
    //capitalizes the first letter of the content
    pub fn capitalize_first_letter(&mut self) {
        self.city = self.city[0..1].to_ascii_uppercase() + &self.city[1..];
        self.country = self.country[0..1].to_ascii_uppercase() + &self.country[1..];
        self.region = self.region[0..1].to_ascii_uppercase() + &self.region[1..];
    }
}
