bool isPalindrome(int x) {    
    int original = x;

    int palindrome = 0; 
    int rem;

    while(x) {        
        rem = x % 10; 
        if (x < 0) return 0;

        palindrome = (palindrome * 10) + rem;

        if (palindrome < (INT_MIN / 10) || (palindrome == INT_MIN / 10 && rem < -8)) {
            break;
        }     

        if (palindrome > (INT_MAX / 10) || (palindrome == INT_MAX / 10 && rem > 7)) {
            break;
        }        

        x = x / 10; 
    }        
    
    if(original == palindrome) return 1;
    else return 0; 
}
