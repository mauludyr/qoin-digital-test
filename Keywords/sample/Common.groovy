
package sample

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper

public class Common {

	public static JsonSlurper jsonSlurper = new JsonSlurper()

	@Keyword
	def int createNewUser(String name, String job, int expectedStatus) {
		def response = WS.sendRequestAndVerify(findTestObject("Object Repository/Create User",
				["name": name, "job": job]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())

		return 7
	}

	@Keyword
	def static void findUserById(int id, String name, String job, int expectedStatus) {
		WS.sendRequestAndVerify(findTestObject('Object Repository/GET user by id', [('id') : id]))
	}

	@Keyword
    def static String findUserEmailById(int id, int expectedStatus) {
        def request = WS.sendRequestAndVerify(findTestObject('Object Repository/GET user by id', [('id') : id]))

        def response = WS.sendRequestAndVerify(request)

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
            
		return jsonResponse.email
    }
}
