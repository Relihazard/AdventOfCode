package main

import (
    "fmt"
    "log"
    "os"
    "bufio"
    "strconv"
    "strings"
)

func main() {
    file, err := os.Open("input")

    if err != nil {
        log.Fatal(err)
        return
    }

    defer file.Close()

    scanner := bufio.NewScanner(file)
    var counter int

    for scanner.Scan() {
        line := scanner.Text()
        splits := strings.Split(line, " ")
        limits := strings.Split(splits[0], "-")
        first_position, _ := strconv.Atoi(limits[0])
        second_position, _ := strconv.Atoi(limits[1])
        c := splits[1][:1]

        if splits[2][first_position - 1] == c[0] && splits[2][second_position - 1] == c[0] {
            continue
        }
        if splits[2][first_position - 1] == c[0] || splits[2][second_position - 1] == c[0] {
            counter++
        }
    }

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
        return
    }

    fmt.Printf("%d\n", counter)
}
