package main

import (
    "fmt"
    "os"
    "bufio"
    "log"
)

func part_one(a []string, line_len int, limit int) {
    var trees, i, j int

    for {
        j += 3
        if j >= line_len {
            j = j - line_len
        }
        i++
        if a[i][j] == '#' {
            trees++
        }
        if i == (limit - 1) {
            break
        }
    }

    fmt.Printf("%d\n", trees)
}

func part_two(a []string, line_len int, limit int) {
    slopes := [][]int{{1, 1}, {1, 3}, {1, 5}, {1, 7}, {2, 1}}
    trees := make([]int, 5)

    for index, value :=  range slopes {
        right := value[1]
        bottom := value[0]
        var i, j int

        for {
            j += right
            if j >= line_len {
                j = j - line_len
            }
            i += bottom
            if a[i][j] == '#' {
                trees[index]++
            }
            if i == (limit - 1) {
                break
            }
        }
    }

    total := 1

    for _, value := range trees {
        total = total * value
    }

    fmt.Printf("%d\n", total)
}

func main() {
    file, err := os.Open("input")

    if err != nil {
        log.Fatal(err)
        return
    }

    defer file.Close()

    scanner := bufio.NewScanner(file)
    var a[]string

    for scanner.Scan() {
        a = append(a, scanner.Text())
    }

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
        return
    }

    line_len := len(a[0])
    limit := len(a)

    part_one(a, line_len, limit)
    part_two(a, line_len, limit)
}
