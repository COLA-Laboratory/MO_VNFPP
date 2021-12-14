//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see http://www.gnu.org/licenses/.
// 

namespace simulation;

message DestMessage {
    int srcServer;
    bool isExternalMessage;
    
    int destination;
    
    int position = 0;	// Which VNF in the service the message is up to
    int hopCount = 0;	// Number of switches/VNFs visited
    int serviceId = 0;	// The ID of the service the message belongs too
    
    bool visitedSDN = false;
    bool hasFinished = false;
    
    simtime_t produced;
    simtime_t queued;
}
