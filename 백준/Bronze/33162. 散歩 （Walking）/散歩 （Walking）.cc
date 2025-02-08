#include <iostream>
using namespace std;

int main()
{
    int n;
    cin >> n;
    int cnt = 0;
    for (int i = 1; i <= n; i++)
    {
        if (i % 2 != 0)
        {
            cnt += 3;
        }
        else
        {
            cnt -= 2;
        }
    }
    cout << cnt << endl;
    return 0;
}