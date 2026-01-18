
/// Using mut slice form access slice of container like Vec or array
/// Make it mut to allow change values
/// The type must be PartialOrd for ( > , < , >= , <=) and Copy to can copy 
/// data 
/// Starting loop on slice
/// len variable save the length of slice( the number of element in the slice )
/// j variable save the starting point 
/// key variable contain starting value which have index j 
/// If the previous value of key is less  we move the value one step forward
/// We make that operation until we don't have previous value  ( j <=0)
/// or key is bigger than previous value of  itself
/// we assign key in current index j
pub fn sort<  T >(  sl:&mut [T] )
    where T: PartialOrd  + Copy
{
    let len: usize = sl.len() ;
   for i in 1..len  {

        let key = sl[i];
        let mut j = i;

        while j > 0 && key < sl[j -1] {
            sl[j] = sl[j -1] ;
            j-=1;
        }
        sl[j] = key ;
   }
}
#[cfg(test)]
mod test_insertion_sort {

    use crate::algorithms;

    #[test]
    fn sort() {
        let mut arr = [1 , 3 , 4 , -4 , -9 ,0 , 2 , 10];
        algorithms::sort::insertion_sort::sort(&mut arr);

        assert_eq!(arr , [ -9 , -4 , 0 , 1 , 2, 3 , 4 , 10]);

        let mut arr = [ -8 , -100 , -7 ,-101];
        algorithms::sort::insertion_sort::sort(&mut arr);

        assert_eq!(arr , [ -101 , -100 , - 8 , -7] );

    }
}

