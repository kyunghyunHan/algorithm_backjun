#include <string>
#include <vector>

using namespace std;

int solution(int n, int k) {
    //10인분을 먹으면 음류수 하나를  줌
    //n= 12000
    //k= 2000
    int answer =  (n>=10)?(n*12000)+((k-(n/10))*2000) : (n*12000)+(k*2000);
    return answer;
}