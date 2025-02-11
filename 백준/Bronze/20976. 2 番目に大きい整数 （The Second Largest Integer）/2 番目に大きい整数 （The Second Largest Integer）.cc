#include <iostream>
#include <vector>
using namespace std;

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    vector<int> abc(3, 0);

    for (int i = 0; i < abc.size(); i++)
    {
        cin >> abc[i];
    }
    sort(abc.begin(), abc.end());

    cout << abc[1] << endl;
    return 0;
}