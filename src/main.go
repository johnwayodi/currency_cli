package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strconv"
	"strings"

	"github.com/urfave/cli/v2"
)

const BASE_URL = "https://open.er-api.com/v6/latest"

type ApiResponse struct {
	Rates map[string]float64 `json:"rates"`
}

func main() {
	app := &cli.App{
		Name:  "currency-cli",
		Usage: "Convert between currencies",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    "convert",
				Usage:   "Convert between currencies in the format FROM-TO, e.g., KES-UGX",
				Aliases: []string{"c"},
			},
		},
		Action: func(c *cli.Context) error {
			convert := c.String("convert")
			if convert != "" {
				currencies := strings.Split(convert, "-")
				if len(currencies) == 2 {
					from := strings.ToUpper(currencies[0])
					to := strings.ToUpper(currencies[1])
					amount := c.Args().Get(0)
					if amount == "" {
						amount = "1"
					}
					convertCurrencies(from, to, amount)
				} else {
					fmt.Println("Invalid format for conversion. Use: FROM-TO")
				}
			} else {
				// Default behavior
				fmt.Println("Enter country codes separated by commas (e.g., 'KES,UGX' for Kenyan Shilling and Ugandan Shilling):")
				var input string
				fmt.Scanln(&input)
				countryCodes := strings.Split(input, ",")
				for i, code := range countryCodes {
					countryCodes[i] = strings.TrimSpace(strings.ToUpper(code))
				}
				getExchangeRates(countryCodes)
			}
			return nil
		},
	}

	err := app.Run(os.Args)
	if err != nil {
		fmt.Println(err)
	}
}

func convertCurrencies(from, to, amount string) {
	url := fmt.Sprintf("%s/%s", BASE_URL, from)
	resp, err := http.Get(url)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	var apiResponse ApiResponse
	err = json.Unmarshal(body, &apiResponse)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	rate, ok := apiResponse.Rates[to]
	if !ok {
		fmt.Printf("Error: Couldn't find exchange rate for %s\n", to)
		return
	}

	amountFloat, err := strconv.ParseFloat(amount, 64)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	converted := amountFloat * rate
	fmt.Printf("%s %s = %.2f %s\n", amount, from, converted, to)
}

func getExchangeRates(countryCodes []string) {
	resp, err := http.Get(BASE_URL)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	var apiResponse ApiResponse
	err = json.Unmarshal(body, &apiResponse)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	for _, countryCode := range countryCodes {
		rate, ok := apiResponse.Rates[countryCode]
		if ok {
			fmt.Printf("1 USD = %.2f %s\n", rate, countryCode)
		} else {
			fmt.Printf("Couldn't find exchange rate for %s\n", countryCode)
		}
	}
}
