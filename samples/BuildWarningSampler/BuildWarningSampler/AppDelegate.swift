//
//  AppDelegate.swift
//  BuildWarningSampler
//
//  Created by Yan Li on 5/14/17.
//  Copyright © 2017 Codezerker. All rights reserved.
//

import UIKit
import WarningKit

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate, UISearchDisplayDelegate {

    var window: UIWindow?

    enum SampleEnum {
        case case1(Int)
        case case2
    }
    
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?) -> Bool {
        let unusedVariable = "unused"
        
        let sampleEnum = SampleEnum.case1(1)
        switch sampleEnum {
        case .case1(let value):
            break
        case .case2:
            break
        }
        
        return true
    }
    
    func searchDisplayControllerDidEndSearch(_ controller: UISearchDisplayController) {
        
    }
}

