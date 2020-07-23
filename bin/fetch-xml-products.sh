#!/usr/bin/env bash

# ZUNKAPATH must be defined.
[[ -z "$ZUNKAPATH" ]] && printf "error: ZUNKAPATH enviorment not defined.\n" >&2 && exit 1 
[[ -z "$ALLNATIONS_USER" ]] && printf "error: ALLNATIONS_USER enviorment not defined.\n" >&2 && exit 1 
[[ -z "$ALLNATIONS_PASS" ]] && printf "error: ALLNATIONS_PASS enviorment not defined.\n" >&2 && exit 1 

# Create dir if not exist.
XML_PATH=$ZUNKAPATH/xml/allnations
mkdir -p $XML_PATH

# Last downloaded XML file.
F_LAST=$XML_PATH/allnations-products.xml
# XML file backup.
F_BACKUP=$XML_PATH/allnations-products-$(date +%Y-%h-%d-%H%M%S).xml
# Last time XML file was downloaded.
LAST_DATE=2020-07-18T09:00:00-03:00

# Download xml file.
# url = 'http://wspub.allnations.com.br/wsIntEstoqueClientes/ServicoReservasPedidosExt.asmx/RetornarListaProdutos?CodigoCliente=000&Senha=000&Data=2016-08-30T09:00:00-03:00';
URL="http://wspub.allnations.com.br/wsIntEstoqueClientesV2/ServicoReservasPedidosExt.asmx/RetornarListaProdutosEstoque?CodigoCliente=${ALLNATIONS_USER}&Senha=${ALLNATIONS_PASS}&Data=${LAST_DATE}"
# echo $URL
curl -v $URL > $F_BACKUP

# Copy as last xml file.
cp $F_BACKUP $F_LAST