using System.Runtime.InteropServices;
using System.Collections.Generic;
using System;
using UnityEngine;

/*
    This class will pool loaded DLL's to save from loading the same DLL multiple times.
    It will also manage the unloading of said DLL's (This must be called however)

    Attach this script to a gameobject.

*/

namespace runity_test
{
    /// <summary>
    /// This class contains some functions from the native windows library
    /// to help us load and release libraries at runtime.
    /// </summary>
    static class NativeMethods
    {
        [DllImport("kernel32.dll")]
        public static extern IntPtr LoadLibrary(string dllToLoad);

        [DllImport("kernel32.dll")]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);

        [DllImport("kernel32.dll")]
        public static extern bool FreeLibrary(IntPtr hModule);
    }

    public class DLLPool: MonoBehaviour
    {
        /// Here we store the Loaded DLL's in a dictionary.
        ///
        /// The key is the DLL's name. This must be a unique, identifying string.
        ///
        /// The value is the DLL's handle (a pointer to the DLL) as well as an integer
        /// which stores the number of currently loaded instances of the DLL. This allows
        /// us to safely unload the DLL when we no longer need it (by ensuring that we've got no more active instances).
        public static Dictionary<string, (IntPtr, int)> dllPool = new Dictionary<string, (IntPtr, int)>();

        private static DLLPool instance;
        public static DLLPool Instance { get { return instance; } }

        private void Awake()
        {
            if (instance != null && instance != this)
            {
                Destroy(this.gameObject);
            }
            else
            {
                instance = this;
            }
        }
        
        /// <summary>
        /// This function loads a DLL from the stored dictionary.
        /// 
        /// This helps you save time by loading DLL's loaded already, rather than load them again.
        /// 
        /// IsLoad should only be set if the DLL is being loaded by a script for the first time.
        /// </summary>
        /// <param name="dllName"></param>
        /// <returns></returns>
        private static IntPtr GetDLL(string dllName, bool isLoad)
        {
            if (dllPool.ContainsKey(dllName))
            {
                var val = dllPool[dllName];
                if (isLoad)
                {
                    val.Item2 = val.Item2 + 1; // New reference, so we increase the count
                    Debug.Log("Living References: " + val.Item2);

                    // reassign the value
                    dllPool[dllName] = val;
                }

                return dllPool[dllName].Item1;
            }

            return IntPtr.Zero;
        }



        /// <summary>
        /// This function takes the name of the DLL to load
        /// 
        /// These DLL's are expected to be stored in the `Assets/Plugins/` folder
        /// </summary>
        /// <param name="dllName"></param>
        /// <returns></returns>
        public static IntPtr LoadDLL(string dllName)
        {
            if (GetDLL(dllName, false) != IntPtr.Zero)
            {
                Debug.Log("DLL Exists! Loading...");
                return GetDLL(dllName, true);
            }
            else
            {
                Debug.Log("DLL does not exist! Loading new...");
            }

            //Get the path of the Game data folder
            string m_Path = Application.dataPath;

            // We load differently based on whether it is the player or editor - this is because
            // the player automatically stores DLL's at the location below
#if UNITY_EDITOR
            string path = m_Path + "/Plugins/" + dllName;
#else
            string path = m_Path + "/Plugins/x86_64/" + dllName;
#endif
            Debug.Log("Loading DLL: " + path);
            // Load the DLL library
            IntPtr loadedDLLPtr = NativeMethods.LoadLibrary(path);
            if (loadedDLLPtr == IntPtr.Zero)
            {
                Debug.LogError("Error! The library " + dllName + " couldn't be found!");
                return IntPtr.Zero;
            }
            else
            {
                Debug.Log("Successfully loaded DLL: " + path);
            }

            // Add the pointer to our pool.
            dllPool.Add(dllName, (loadedDLLPtr, 1));

            return loadedDLLPtr;
        }

        /// <summary>
        /// This function will load a function from a library. It takes in the DLL name (so it can find the DLL from the pool), the function
        /// name and the function delegate - this defines the structure of function for c# to use.
        /// </summary>
        /// <param name="dllName"></param>
        /// <param name="functionName"></param>
        /// <param name="delegateType"></param>
        /// <returns></returns>
        public static (Delegate, IntPtr) LoadFunctionFromDLL(string dllName, string functionName, Type delegateType)
        {
            try
            {
                IntPtr dllPtr = (IntPtr)0;
                // We check that the DLL is loaded. If it isn't, throw an exception. Once we've loaded it, we then load the function.
                if (GetDLL(dllName, false) != IntPtr.Zero)
                {
                    Debug.Log("DLL exists in pool, loading from pool...");
                    dllPtr = GetDLL(dllName, false);
                }
                else
                {
                    throw new Exception("Error - DLL not loaded");
                }

                IntPtr functionPtr = NativeMethods.GetProcAddress(dllPtr, functionName);
                if (functionPtr == IntPtr.Zero)
                {
                    Debug.LogWarning("Couldn't find function '" + functionName + "' in " + dllName + ", '" + functionName + "' in unity won't be run!");
                    throw new ExternalException("Error - function failed to load. Maybe it isn't present or has a different name?");
                }
                Delegate function = Marshal.GetDelegateForFunctionPointer(functionPtr, delegateType);
                Debug.Log("Loaded function '" + functionName + "' from " + dllName + " successfully!");
                return (function, functionPtr);
            }
            catch (Exception e)
            {
                Debug.Log("Error when loading function from DLL - " + e);
                return (null, IntPtr.Zero);
            }
        }

        /// <summary>
        /// Unload all will unload a given DLL
        /// 
        /// Should be run on exit or on destruction.
        /// </summary>
        public static void UnloadDLL(string dllName)
        {
            Debug.Log("Attempting to unload DLL: " + dllName);
            if (dllPool.ContainsKey(dllName))
            {
                // Check that the reference count is equal to 0 (item2)
                var val = dllPool[dllName];
                if (val.Item2 == 1)
                {
                    Debug.Log("DLL exists in pool, releasing...");
                    NativeMethods.FreeLibrary(dllPool[dllName].Item1);
                    dllPool.Remove(dllName);
                    Debug.Log("DLL successfully released!");
                }
                else
                {
                    Debug.Log("DLL references still loaded: " + val.Item2);
                    val.Item2 -= 1;
                }

            }
        }
    }
}


