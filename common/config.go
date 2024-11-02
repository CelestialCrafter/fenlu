package common

import (
	"reflect"

	"github.com/BurntSushi/toml"
)

type ConfigStructure struct {
	BatchSize int `toml:"batch_size"`
}

var Config ConfigStructure
var Default = ConfigStructure {
	BatchSize: 1024,
}

const ConfigPath = "config.toml"

func LoadConfig() (ConfigStructure, error) {
	_, err := toml.DecodeFile(ConfigPath, &Config)
	if err != nil {
		return ConfigStructure{}, err
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

	return Config, nil

}

