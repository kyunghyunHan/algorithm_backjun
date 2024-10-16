#include <iostream>
#include <unordered_map>
#include <algorithm>

using namespace std;

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    int n(0), idx(0);
    cin >> n;
    string name;
    unordered_map<string, int> runner;
    for (int i = 0; i < n; i++)
    {
        cin >> name;
        runner[name]++;
    }

    for (idx = 0; idx < n - 1; idx++)
    {
        cin >> name;
        runner[name]--;
    }

    unordered_map<string, int>::iterator iter;

    for (iter = runner.begin(); iter != runner.end(); iter++)
    {
        if (iter->second != 0)
        {
            cout << iter->first;
            return 0;
        }
    }
   
}