<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update User</name>
   <tag></tag>
   <elementGuidId>986e6ac4-88a3-46b4-9c30-2bb4939aaed7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;job\&quot;: \&quot;${job}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f25373f7-8b37-4f51-b019-13c602e722b6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/users/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>7</defaultValue>
      <description></description>
      <id>c405e5dd-993f-4852-a458-f66c43958606</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'user-edit'</defaultValue>
      <description></description>
      <id>d3ca0a53-26c7-492c-b335-e90ddab99b79</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'freelance'</defaultValue>
      <description></description>
      <id>36d9e956-d861-4ad5-bd73-e327171d55cd</id>
      <masked>false</masked>
      <name>job</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
