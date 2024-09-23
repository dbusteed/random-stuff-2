#
#  Make a SOAP request with Python
#

from zeep import Client
from zeep.helpers import serialize_object
import requests
import re


#
#  APPROACH 1: Using `zeep` module
#
wsdl = 'http://dneonline.com/calculator.asmx?wsdl'

client = Client(wsdl)

response = serialize_object(
    client.service.Add(intA=5, intB=7)
)

print(response)


#
#  APPROACH 2: Using raw XML
#

xml = """
  <?xml version='1.0' encoding='utf-8'?>
  <soap-env:Envelope xmlns:soap-env="http://schemas.xmlsoap.org/soap/envelope/">
    <soap-env:Body>
      <ns0:Add xmlns:ns0="http://tempuri.org/">
        <ns0:intA>5</ns0:intA>
        <ns0:intB>7</ns0:intB>
      </ns0:Add>
    </soap-env:Body>
  </soap-env:Envelope>
"""

xml = re.sub(r'\n\s*', '', xml)

r = requests.post(wsdl, data=xml, headers={'Content-Type': 'text/xml'})

print(r.text)
