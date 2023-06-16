module.exports = {
	"overrides": [
		{
		  "files": ["src/**/*.ts"],
		}
	],
	"ignorePatterns": ["*.js", "*.json"],
	"env": {
		"browser": true,
		"commonjs": true,
		"es2021": true
	},
	"extends": [
		"eslint:recommended",
		"plugin:@typescript-eslint/recommended"
	],
	"parser": "@typescript-eslint/parser",
	"parserOptions": {
		"ecmaVersion": "latest",
		"sourceType": 'module',
		"project": './tsconfig.json',
	},
	"plugins": [
		"@typescript-eslint"
	],
	"rules": {
		"indent": [
			"warn",
			2
		],
		"linebreak-style": [
			"warn",
			"windows"
		],
		"quotes": [
			"warn",
			"double"
		],
		"semi": [
			"warn",
			"always"
		]
	}
};
