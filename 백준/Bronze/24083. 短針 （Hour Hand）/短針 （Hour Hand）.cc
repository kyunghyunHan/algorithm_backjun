#include <iostream>

using namespace std;
int main()
{
    int a, b;
    cin >> a;
    cin >> b;

    int result = (a + b) % 12;
    if (result == 0)
    {
        cout << 12 << endl;
    }
    else
    {
        cout << result << endl;
    }
}