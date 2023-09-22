func solution(numbers []int) float64 {
    
    sum := 0
    for i := 0; i < len(numbers); i++ {
		sum += numbers[i]
	}
    result:= float64(sum)/float64(len(numbers))
    return result
}