package config

import (
	"reflect"

	"github.com/BurntSushi/toml"
)

type config struct {
	BatchSize int `toml:"batch_size"`
	Nodes map[string]any `toml:"nodes"`
}

var Config config
var Default = config {
	BatchSize: 1024,
	Nodes: map[string]any{},
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

	return nil

}

