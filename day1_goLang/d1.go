package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
)

func calculateSimilarityScore(leftList, rightList []int) int {
	rightCounts := make(map[int]int)
	for _, num := range rightList {
		rightCounts[num]++
	}

	similarityScore := 0
	for _, num := range leftList {
		similarityScore += num * rightCounts[num]
	}

	return similarityScore
}

func readListsFromCSV(filePath string) ([]int, []int, error) {
	leftList := []int{}
	rightList := []int{}

	file, err := os.Open(filePath)
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()

	reader := csv.NewReader(file)
	records, err := reader.ReadAll()
	if err != nil {
		return nil, nil, err
	}

	for _, row := range records {
		if len(row) >= 2 {
			leftNum, err1 := strconv.Atoi(row[0])
			rightNum, err2 := strconv.Atoi(row[1])
			if err1 == nil && err2 == nil {
				leftList = append(leftList, leftNum)
				rightList = append(rightList, rightNum)
			}
		}
	}

	return leftList, rightList, nil
}

func main() {
	filePath := "lists.csv"

	leftList, rightList, err := readListsFromCSV(filePath)
	if err != nil {
		fmt.Println("Error reading the file:", err)
		return
	}

	similarityScore := calculateSimilarityScore(leftList, rightList)
	fmt.Printf("The similarity score is: %d\n", similarityScore)
}
