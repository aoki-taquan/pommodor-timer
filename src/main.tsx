import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";

import { extendTheme, type ThemeConfig } from '@chakra-ui/react'
import { ChakraProvider, useColorMode } from '@chakra-ui/react'

// 2. Add your color mode config
const config: ThemeConfig = {
  initialColorMode: 'dark',
  useSystemColorMode: false,
}

// 3. extend the theme
const theme = extendTheme({ config })

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(

    <ChakraProvider theme={extendTheme(config)}>
      <App />
    </ChakraProvider>

);
