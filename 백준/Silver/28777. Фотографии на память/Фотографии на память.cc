#include <iostream>
#include <vector>

using namespace std;

int main()
{
    int n;

    cin >> n;
    vector<int> heights(n);

    for (int i = 0; i < n; i++)
    {
        cin >> heights[i];
    }

    sort(heights.begin(), heights.end());

    vector<int> dp(n + 1, n);
    dp[0] = 0;

    for (int i = 0; i < n; i++)
    {
        dp[i + 1] = dp[i] + 1;
        if (i > 0 && heights[i] - heights[i - 1] <= 20)
        {
            dp[i + 1] = min(dp[i + 1], dp[i - 1] + 1);
        }

        if (i > 1 && heights[i] - heights[i - 2] <= 10)
        {
            dp[i + 1] = min(dp[i + 1], dp[i - 2] + 1);
        }
    }

    cout << dp[n] <<endl;
}
