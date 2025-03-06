#include <iostream>

using namespace std;
int n, h;
int a[201];
int main()
{
    cin >> n >> h ;
    int count = 0;
    for (int i = 1; i<=n;i++){
        cin >>a[i];
        if (h >= a[i]){
            count+=1;
        }
    };

    cout << count << endl;
    return 0;
}