#include <iostream>
#include <iomanip>
#include <vector>
#include <cmath>

using namespace std;

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    float n;
    cin >> n;

    cout << fixed << setprecision(8) << powf(n, 0.5) * 4 << endl;
    return 0;
}
