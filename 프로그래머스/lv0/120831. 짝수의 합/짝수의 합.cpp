#include <string>
#include <vector>

using namespace std;

int solution(int n) {
      int answer = 0;
   for (int count = 1; count <= n; ++count)
     if(count%2 ==0)
         answer +=count;
  
    return answer;
}