#include <iostream>
#include <string>
using namespace std;

int main(){
    string s;
    cin >> s;

    int e_count = count(s.begin(),s.end(),'e');

    string result = "h" + string (e_count *2,'e')  + 'y';
    cout << result << endl;
    return 0;

}