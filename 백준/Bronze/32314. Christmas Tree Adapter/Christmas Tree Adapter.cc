#include <iostream> 
using namespace std;
 
int main(void)
{
    int a;
    double w, v;

    cin >> a >> w >> v;

    cout << (w / v >= a ? 1 : 0) << endl;

    return 0;
}
