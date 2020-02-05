package main

import (
	"fmt"
	"html/template"
	"log"
	"math/rand"
	"net/http"
	"time"
)

const charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

var seededRand *rand.Rand = rand.New(rand.NewSource(time.Now().UnixNano()))

func main() {
	fmt.Println(stringWithCharset(32, charset))
	http.HandleFunc("/", HomePageHandler)
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func stringWithCharset(length int, charset string) string {
	b := make([]byte, length)
	for i := range b {
		b[i] = charset[seededRand.Intn(len(charset))]
	}
	return string(b)
}

type PageVariables struct {
	Date string
	Time string
}

func HomePageHandler(w http.ResponseWriter, r *http.Request) {

	now := time.Now()              
	HomePageVars := PageVariables{ 
		Date: now.Format("02-01-2006"),
		Time: now.Format("15:04:05"),
	}

	t, err := template.ParseFiles("index.html")
	if err != nil {                                
		log.Print("template parsing error: ", err) 
	}
	err = t.Execute(w, HomePageVars) 
	if err != nil {                  
		log.Print("template executing error: ", err)
	}
}
