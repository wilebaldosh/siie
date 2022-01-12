package db

import (
	"fmt"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func Info() {
	fmt.Println("db::Info")
}

func OpenDB() {
	db, err := gorm.Open(sqlite.Open("siie.db"), &gorm.Config{})
	if err != nil {
		panic("Faile to connect database")
	}
	fmt.Println(typeof(db))
}

func Create(db) {
}
