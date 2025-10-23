#define Iterator typename
#define Integer typename

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

template <Iterator I, Integer N>

void mark_sieve(I first, I last, N factor) {
    
    // first != last:  last - first = qty of elements
      
    *first = false;
    
    while (last - first > factor) {
        first = first + factor;
        *first = false;
    }
}


template <Iterator I, Integer N>

void sift(I first, N n) {
    
    //fill(first, first + n, true); original
   
    N i{0};
    N index_square{3};
    
    while (index_square < n) {
        
        //index_square = 2*i^2 + 6*i + 3
        
        if (first[i]) { // maybe *(first + i)

           mark_sieve(first + index_square, first + n, i + i + 3);
            
        }
                
        ++i;
        index_square = 2 * i * (i + 3) + 3;        
        
    }
    
}


int main() {
    
    long n{1000};
    
    vector<bool> primes(n, true);
    
    //fill(primes.begin(), primes.end(), true); no need: init
    
    long l{1};
    
    cout << "2 ";
    
    sift(primes.begin(), n);
    
    for (long i = 0; i < primes.size(); ++i) {
        
        if (primes[i]) {
            
            cout << 2*i + 3<< " ";
            
            ++l;
            
            if (l % 10 == 0) cout << "\n"; 
        }
        
    }
    
    cout << "\nnumber : " << l << "\n";  
    
}
