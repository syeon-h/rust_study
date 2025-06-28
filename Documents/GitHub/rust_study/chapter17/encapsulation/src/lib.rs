/*
    OOP languages share certein common charactersitics: 
        namely objects, encapsulation, and inheritance; 
        object packages both data and the procedures that operate on 
        that data; procedures are typically called methods or operations 

        Rust is object-oriented in this definition: structs and enums 
        have data and impl blocks provide methods on them 

    encapsulation means that the implementation details of an object 
        are not accessible to code using that object; thus, the only way 
        to interact is through its public API and code using the object 
        should not be able to reach into the internals (data) directly 

        recall: pub keyword decide which modules, types, functions, and 
            methods in the code should be public (private by default) 

    Ex. AveragedCollection is marked pub so that other code can use it, 
        but the feilds within remain private; to ensure that whenever 
        a value is added or removed, the average is also updated 

        public methods add, remove, and average are the only ways to 
        access or modify data in an instance; when an item is added or 
        removed, the private update_average handles the average field 

    because we have encapsulated the implementation details, we can easily
        change aspects, such as the data strcuture, in the future 

        Ex. use a HashSet<i32> instead of a Vec<i32> for the list field: 
        code using AveragedCollection would not need to change; but if 
        list is public instead, HashSet<i32> and Vec<i32> have different 
        methods for adding and removing items, so the external code would 
        likely have to change if it were modifying list directly 
*/

// cache the calculated average: 
pub struct AveragedCollection {
    list: Vec<i32>, 
    average: f64, 
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value); 
        self.update_average(); 
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop(); 
        match result {
            Some(value)  => {
                self.update_average();
                Some(value)
            }
            None => None, 
        }
    }

    pub fn average(&self) -> f64 {
        self.average 
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum(); 
        self.average = total as f64 / self.list.len() as f64; 
    }
}
