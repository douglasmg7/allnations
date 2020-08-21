#!/usr/bin/env bash

# ZUNKAPATH must be defined.
[[ -z "$ZUNKAPATH" ]] && printf "error: ZUNKAPATH enviorment not defined.\n" >&2 && exit 1 
[[ -z "$ALLNATIONS_USER" ]] && printf "error: ALLNATIONS_USER enviorment not defined.\n" >&2 && exit 1 
[[ -z "$ALLNATIONS_PASS" ]] && printf "error: ALLNATIONS_PASS enviorment not defined.\n" >&2 && exit 1 

# Create dir if not exist.
XML_PATH=$ZUNKAPATH/xml/allnations
mkdir -p $XML_PATH

NOW=$(date +%Y-%m-%dT%H:%M:%S-03:00)

# Last downloaded XML file.
FILE_LAST=$XML_PATH/allnations_products_to_process.xml

# Will not download products information if last file not processed yet.
if [ -f $FILE_LAST ]; then
    echo "Nothing to do, last file wasn't processed yet."
    exit 0
fi

FILE_LAST_TIME_PRODUCT_WAS_DOWNLOADED=$XML_PATH/allnations_products_last_time_download.time
if [[ -f $FILE_LAST_TIME_PRODUCT_WAS_DOWNLOADED ]]; then
    LAST_TIME_PRODUCT_WAS_DOWNLOADED=$(cat $FILE_LAST_TIME_PRODUCT_WAS_DOWNLOADED)
    echo Last time products was downloaded: $LAST_TIME_PRODUCT_WAS_DOWNLOADED
else
    LAST_TIME_PRODUCT_WAS_DOWNLOADED=2018-01-01T03:00:00-03:00
    echo Products was never downloaded, using: $LAST_TIME_PRODUCT_WAS_DOWNLOADED
fi

# XML file backup.
FILE_BACKUP=$XML_PATH/allnations_products_${NOW}.xml

# Download xml file.
URL="http://wspub.allnations.com.br/wsIntEstoqueClientesV2/ServicoReservasPedidosExt.asmx/RetornarListaProdutosEstoque?CodigoCliente=${ALLNATIONS_USER}&Senha=${ALLNATIONS_PASS}&Data=${LAST_TIME_PRODUCT_WAS_DOWNLOADED}"
curl -v $URL > $FILE_BACKUP
# echo asdfadsfasdfasdfasdfasdfasdçflkajsdçflajsçdlfkjasdlkfjaçsdlkfjaçlsdkfjçalskdjfçalksdjçflaksjdçflkajsçdlfkajçsdlfkjaçsldkfjçalsdkfjçalskdjfasldfkj > $FILE_BACKUP

# Get created file size.
if [[ -f $FILE_BACKUP ]]; then
    FILE_SIZE=$(stat -c%s $FILE_BACKUP)
else
    FILE_SIZE=0
fi

# If a valid size, copy as last xml file to process.
# echo size: $FILE_SIZE
if [[ "$FILE_SIZE" -gt "100" ]]; then
    cp $FILE_BACKUP $FILE_LAST
    echo $NOW > $FILE_LAST_TIME_PRODUCT_WAS_DOWNLOADED
fi