import * as readline from 'readline';

// readline 인터페이스 설정
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

// 사용자 입력을 받을 변수들
let budget: number;
let watermelonPrice: number;
let pomegranatesPrice: number;
let nutsPrice: number;

// 입력 받기
rl.question('', (input) => {
    budget = parseInt(input);  // 예산
    rl.question('', (input) => {
        watermelonPrice = parseInt(input);  // 수박 가격
        rl.question('', (input) => {
            pomegranatesPrice = parseInt(input);  // 석류 가격
            rl.question('', (input) => {
                nutsPrice = parseInt(input);  // 견과류 가격
                
                // 계산 함수 호출
                const result = selectItem(budget, watermelonPrice, pomegranatesPrice, nutsPrice);
                
                // 결과 출력
                console.log(result);
                
                // readline 종료
                rl.close();
            });
        });
    });
});

// 아이템을 선택하는 함수
function selectItem(budget: number, watermelon: number, pomegranates: number, nuts: number): string {
    if (budget >= watermelon) {
        return "Watermelon";
    } else if (budget >= pomegranates) {
        return "Pomegranates";
    } else if (budget >= nuts) {
        return "Nuts";
    } else {
        return "Nothing";
    }
}
