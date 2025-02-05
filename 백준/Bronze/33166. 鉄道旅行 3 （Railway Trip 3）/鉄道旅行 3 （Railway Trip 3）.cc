#include <iostream>
using namespace std;


int main() 
{
 int p,q ,a ,b;

 cin >> p >> q;
 cin >> a  >>b;
 if (q-p <= 0) {
 cout << q * a ;

 }else{
 cout << p * a + (q - p) * b;

 }
 return 0;
}

