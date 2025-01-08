use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // 순위 점수표
    let rank_points = [10, 8, 6, 5, 4, 3, 2, 1];

    // 시간을 레이서 순서대로 저장
    let mut racers: Vec<(u32, char)> = Vec::new();

    // 입력 처리
    for _ in 0..8 {
        let line = input.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let time = parts[0];
        let team = parts[1].chars().next().unwrap();

        // 시간(M:SS:sss)을 밀리초로 변환
        let time_parts: Vec<u32> = time
            .split([':', '.'].as_ref())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let total_time = time_parts[0] * 60_000 + time_parts[1] * 1_000 + time_parts[2];

        racers.push((total_time, team));
    }

    // 시간을 기준으로 정렬
    racers.sort_by_key(|x| x.0);

    // 팀 점수 합산
    let mut red_score = 0;
    let mut blue_score = 0;

    for (rank, (_, team)) in racers.iter().enumerate() {
        let points = rank_points[rank];
        if *team == 'R' {
            red_score += points;
        } else {
            blue_score += points;
        }
    }

    // 승리 팀 판단
    let winner = if red_score > blue_score {
        "Red"
    } else if red_score < blue_score {
        "Blue"
    } else {
        // 점수가 같을 경우 최고 순위 비교
        let top_red_rank = racers.iter().position(|&(_, team)| team == 'R').unwrap();
        let top_blue_rank = racers.iter().position(|&(_, team)| team == 'B').unwrap();

        if top_red_rank < top_blue_rank {
            "Red"
        } else {
            "Blue"
        }
    };

    println!("{}", winner);
}
