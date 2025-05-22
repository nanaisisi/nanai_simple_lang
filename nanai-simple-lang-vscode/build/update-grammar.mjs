/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
//@ts-check

import * as vscodeGrammarUpdater from "vscode-grammar-updater";

vscodeGrammarUpdater.update(
	"dustypomerleau/nasl-syntax",
	"syntaxes/nasl.tmLanguage.json",
	"./syntaxes/nasl.tmLanguage.json",
	undefined,
	"main",
);
