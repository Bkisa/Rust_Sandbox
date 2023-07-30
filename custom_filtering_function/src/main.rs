fn main() {
    // create a collection with elements
    let collection = vec!["Ali", "Mehmet", "Kamil", "Cabbar", "Osman"];

    // Initilize a FilterCondition object with the desired value
    let filter = FilterCondition {
        condition: "Mehmet",
    };

    // Call the custom_filter function with the collection and the FilterCondition and store the result in a new vector
    let filtered_result = custom_filer(&collection, &filter);

    println!("Filtered Result: {:?}", filtered_result);
}

// Define a struct called FilterCondition with a single field of the desired type for filtering
struct FilterCondition<T> {
    condition: T,
}

// Implement the is_match method on the FilterCondition struct.
impl<T: PartialEq> FilterCondition<T> {
    // This method takes a reference to an item and checks if it matches the condition
    fn is_match(&self, item: &T) -> bool {
        // Compare the item with the condition and return true if they are equal
        item == &self.condition
    }
}

// Define the custon_filter function to filter elements based on the condition
fn custom_filer<T>(collection: &[T], filter: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    // Iterate over the collection and filter the elements based on the condition
    collection
        // Create an iterator over the elements of the collection.
        .iter()
        // Clone each element to create a new collection.
        .cloned()
        // Use the filter() method to only keep elements that match the condition.
        .filter(|item| filter.is_match(item))
        // Collect the filtered elements into a new vector a return it.
        .collect()
}
