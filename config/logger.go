package config

import "github.com/charmbracelet/log"

func SetupLogger() {
	log.SetLevel(Config.LogLevel.Level)
	log.SetReportCaller(true)
}
