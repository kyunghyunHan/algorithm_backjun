#include <iostream>

using namespace std;

int main()
{
    int N, K;
    cin >> N >> K;

    int count = 0;
    string k_str = to_string(K); // K를 문자열로 변환 (최적화)

    for (int hour = 0; hour <= N; hour++) // 0시부터 N시까지 포함
    {
        string h = (hour < 10 ? "0" : "") + to_string(hour); // 두 자리로 변환

        for (int minute = 0; minute < 60; minute++)
        {
            string m = (minute < 10 ? "0" : "") + to_string(minute);

            for (int second = 0; second < 60; second++)
            {
                string s = (second < 10 ? "0" : "") + to_string(second);

                // HH:MM:SS 문자열을 생성하여 K 포함 여부 확인
                if (h.find(k_str) != string::npos ||
                    m.find(k_str) != string::npos ||
                    s.find(k_str) != string::npos)
                {
                    count++;
                }
            }
        }
    }

    cout << count << endl;
    return 0;
}
