package config

import (
	"reflect"

	"github.com/BurntSushi/toml"
	"github.com/charmbracelet/log"
)

type node struct {
	Command string `toml:"command"`
	Config any `toml:"config"`
}

type pipeline struct {
	Sources []string `toml:"sources"`
	Sinks []string `toml:"sinks"`
	Processors []string `toml:"processors"`
}

type logLevel struct {
	log.Level
}

func (l *logLevel) UnmarshalText(text []byte) error {
	var err error
	l.Level, err = log.ParseLevel(string(text))
	return err
}

type config struct {
	BatchSize int `toml:"batch_size"`
	BufferSize int `toml:"buffer_size"`
	Pipeline pipeline `toml:"pipeline"`
	LogLevel logLevel `toml:"log_level"`
	Nodes map[string]node `toml:"nodes"`
}

var Config config
var Default = config {
	BatchSize: 1024,
	BufferSize: 10,
	Nodes: map[string]node{},
	Pipeline: pipeline{},
	LogLevel: logLevel{Level: log.InfoLevel},
}

const configPath = "config.toml"

func LoadConfig() error {
	_, err := toml.DecodeFile(configPath, &Config)
	if err != nil {
		return err
	}

	// set default values for keys not found in options file
	t := reflect.ValueOf(&Config).Elem()
	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		if !f.IsZero() {
			continue
		}

		f.Set(reflect.ValueOf(Default).Field(i))
	}

	log.Debug("config loaded", "config", Config)
	return nil
}

