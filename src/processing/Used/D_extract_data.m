addpath(".");

close all
clear
clc

format long

%% load path
root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, 'data', 'Simulation');
out_folder = fullfile(root_path, 'results', 'Simulation');

cd (out_folder);
delete SIMULATION_*.out

cd (src_folder);

test_problems = dir;
test_problems = test_problems(~ismember({test_problems.name},{'.','..', 'parameters'}));

file_runs = zeros(length(test_problems), 1);

test_group_ptrn = '-';

for i = 1 : length(test_problems)
    temp = regexp(test_problems(i).name, test_group_ptrn, 'split');
    test_groups(i) = temp(1);
end

test_groups = unique(test_groups);

field_ptrn = '(?<=field mean )((nan)|(-nan)|[0-9.e-]+)';
energy_ptrn = '(?<=scalar FatTree Energy_Consumption_Total:last )((nan)|(-nan)|[0-9.e-]+)';

for i = 1 : length(test_groups)
    test_group = test_groups{i};
    test_files = dir([test_group, '*.sca']);
    test_files = natsortfiles({test_files.name});
    
    for j = 1 : size(test_files, 2)
        test_file = test_files(j);
        test_file = test_file{1};
        arr_rate = extract_arrival_rate(test_file);
          
        fread = fullfile(src_folder, test_file);
        fread = fileread(fread);
        
        values = regexp(fread, field_ptrn, 'match');
        values = str2double(values);
        values(isnan(values)) = 0;
        
        pl = values(3);
        latency = values(4);
        
        energy = regexp(fread, energy_ptrn, 'match');
        energy = str2double(energy);
        energy(isnan(energy)) = 0;
        
        fwrite = fullfile(out_folder, ['SIMULATION_' test_group '.out']);
        fwrite = fopen(fwrite, 'a');
        
        fprintf(fwrite, '%f %f %f %f \n', arr_rate, latency, pl, energy);
        
        fclose(fwrite);
    end
end

function arrival_rate = extract_arrival_rate(str)
dash_pos = strfind(str, '-');

arrival_rate = str(dash_pos(1)+1 : dash_pos(2)-1);
arrival_rate = str2double(arrival_rate);
end